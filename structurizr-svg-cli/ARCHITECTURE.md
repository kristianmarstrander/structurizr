# Architecture

This document describes the design and architecture of the Structurizr SVG CLI tool.

## High-Level Architecture

```
┌─────────────┐
│   DSL File  │
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│   Parser    │────>│  Workspace  │
│   Module    │     │    Model    │
└─────────────┘     └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Layout    │
                    │   Engine    │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │     SVG     │
                    │  Renderer   │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  SVG Files  │
                    └─────────────┘
```

## Module Structure

### 1. Parser Module (`src/parser/`)

**Responsibility**: Parse Structurizr DSL files into an internal model.

**Components**:
- `tokens.rs` - Token definitions for lexical analysis
- `dsl.rs` - Main parser implementation

**Approach**:
1. **Tokenization**: The DSL content is scanned character-by-character to produce tokens
   - Keywords: `workspace`, `model`, `views`, `person`, `softwareSystem`, etc.
   - Literals: strings, identifiers
   - Operators: `->`, `=`
   - Delimiters: `{`, `}`
   
2. **Parsing**: Recursive descent parser processes tokens
   - `parse_workspace()` - Top-level workspace parsing
   - `parse_model()` - Parse model elements and relationships
   - `parse_element()` - Parse individual elements (person, system, etc.)
   - `parse_views()` - Parse view definitions
   - `parse_styles()` - Parse style definitions

3. **AST Construction**: Build workspace model as tokens are parsed
   - Elements stored in HashMap by ID
   - Relationships stored as list
   - Views reference element IDs
   - Styles stored by tag name

**Design Decisions**:
- Custom parser (not using parser combinators) for better error messages
- Two-pass approach not needed since forward references are resolved by IDs
- Simple variable tracking for relationship definitions

### 2. Model Module (`src/model/`)

**Responsibility**: Data structures representing the Structurizr model.

**Components**:
- `element.rs` - Element types and definitions
- `relationship.rs` - Relationship definitions
- `view.rs` - View types and configurations
- `style.rs` - Style definitions (element and relationship)
- `mod.rs` - Workspace aggregation

**Key Types**:

```rust
Workspace {
    name, description,
    elements: HashMap<String, Element>,
    relationships: Vec<Relationship>,
    views: Vec<View>,
    styles: Styles
}

Element {
    id, name, description,
    element_type: ElementType,
    tags: Vec<String>,
    parent_id, children
}

View {
    key, view_type,
    elements: Vec<String>,
    relationships: Vec<String>,
    auto_layout: Option<AutoLayout>
}
```

**Design Decisions**:
- Elements use String IDs for flexible referencing
- Tags-based styling allows hierarchical style application
- Views contain element IDs rather than references for simplicity
- Workspace owns all elements to avoid lifetime complexity

### 3. Renderer Module (`src/renderer/`)

**Responsibility**: Generate SVG output from the model.

**Components**:
- `svg.rs` - Main SVG renderer
- `shapes.rs` - Shape rendering for all 18 shape types
- `layout.rs` - Auto-layout engine
- `text.rs` - Text wrapping and rendering utilities

**Data Flow**:

```
Workspace + View
      │
      ▼
   Layout                 Calculate positions for elements
      │
      ▼
   Relationships          Render relationship lines/arrows first
      │                   (so they appear behind elements)
      ▼
   Elements               Render element shapes and text
      │
      ▼
   SVG Document           Combine into final SVG
```

**Key Algorithms**:

#### Layout Algorithm (Layered/Hierarchical)

```
1. Build directed graph from elements and relationships
2. Find root nodes (no incoming edges)
3. BFS traversal to assign layers
4. Calculate positions:
   - X position based on layer and direction
   - Y position based on position within layer
5. Store positions in HashMap<ElementId, Position>
```

**Layout Parameters**:
- Node spacing: 100px horizontal, 150px vertical
- Default node size: 450x300px
- Direction affects coordinate calculation

#### Shape Rendering

Each shape is rendered as SVG primitives:
- **Box/RoundedBox**: `<rect>` with optional `rx/ry`
- **Circle/Ellipse**: `<ellipse>`
- **Hexagon/Diamond**: `<polygon>`
- **Cylinder/Pipe**: `<path>` with elliptical arcs
- **Person/Robot**: `<g>` group with multiple primitives
- **Complex shapes**: Combination of primitives

**Text Rendering**:
1. Wrap text to fit within element width
2. Calculate total text height
3. Center vertically within element
4. Render multiple `<text>` elements for each line

**Relationship Rendering**:
1. Calculate edge intersection points (not center-to-center)
2. Draw line with arrow marker
3. Add label at midpoint

**Design Decisions**:
- Use `svg` crate for type-safe SVG generation
- Shapes implemented as static methods returning `Box<dyn Node>`
- Layout uses petgraph for graph algorithms
- Simple edge point calculation (rectangle intersection)

### 4. Utils Module (`src/utils/`)

**Responsibility**: Utility functions.

**Components**:
- `color.rs` - Color validation and conversion

### 5. CLI Module (`src/main.rs`)

**Responsibility**: Command-line interface.

**Uses**:
- `clap` for argument parsing
- `env_logger` for logging
- Calls into library functions

## SVG Generation Pipeline

```rust
fn render_view(workspace, view) -> SVG {
    // 1. Calculate layout
    let layout = Layout::calculate(workspace, view);
    
    // 2. Create SVG document
    let mut doc = Document::new();
    
    // 3. Render relationships (background layer)
    for rel in view.relationships {
        doc.add(render_relationship(rel, layout));
    }
    
    // 4. Render elements (foreground layer)
    for element in view.elements {
        doc.add(render_element(element, layout));
    }
    
    // 5. Return SVG string
    doc.to_string()
}
```

## Style Application

Styles are applied hierarchically based on tags:

```
1. Start with default style
2. Apply "Element" tag style (base style)
3. Apply element type style ("Person", "Software System")
4. Apply custom tags in order
5. More specific styles override general ones
```

Example:
- Element has tags: ["Element", "Person", "External"]
- Applied styles: Element -> Person -> External
- Each style merges/overrides previous properties

## Extension Points

### Adding New Shapes

1. Add enum variant to `Shape` in `model/style.rs`
2. Implement `render_*` method in `renderer/shapes.rs`
3. Update `Shape::from_str()` for DSL parsing
4. Add corresponding case in `ShapeRenderer::render_shape()`

### Adding New View Types

1. Add enum variant to `ViewType` in `model/view.rs`
2. Update parser in `parser/dsl.rs` to handle new view syntax
3. Renderer automatically handles new view types

### Improving Layout

The layout algorithm is pluggable:
1. Implement new layout strategy in `renderer/layout.rs`
2. Keep the same `Layout` interface
3. Optionally add CLI option to select algorithm

### Adding Export Formats

To add PNG/PDF export:
1. Add new renderer module (e.g., `renderer/png.rs`)
2. Use `resvg` or similar for SVG->raster conversion
3. Add format option to CLI

## Performance Considerations

- **Parser**: Single-pass O(n) parsing
- **Layout**: O(n + m) where n=nodes, m=edges
- **Rendering**: O(n) for elements, O(m) for relationships
- **Memory**: Entire workspace held in memory (suitable for typical DSL files)

**Optimization Opportunities**:
- Parallel rendering of independent views
- Caching parsed stylesheets
- Incremental updates (watch mode)

## Error Handling

- **Parser Errors**: Include line/column numbers
- **Validation**: Check element references exist
- **Graceful Degradation**: Continue rendering even if some elements have issues
- **Logging**: Use `log` crate for different verbosity levels

## Testing Strategy

1. **Unit Tests**: Individual module testing
   - Parser: Test each parse function
   - Shapes: Test each shape renderer
   - Layout: Test graph algorithms

2. **Integration Tests**: End-to-end testing
   - Parse sample DSL files
   - Generate SVG output
   - Validate SVG structure

3. **Snapshot Tests**: Using `insta` crate
   - Compare generated SVG against known-good output
   - Detect regressions in rendering

## Dependencies

### Core Dependencies
- `svg` (0.18) - SVG generation
- `petgraph` (0.6) - Graph data structures
- `clap` (4.5) - CLI parsing
- `anyhow` (1.0) - Error handling
- `thiserror` (1.0) - Custom error types
- `log` (0.4) - Logging
- `env_logger` (0.11) - Logging implementation

### Dev Dependencies
- `pretty_assertions` (1.4) - Better test output
- `insta` (1.39) - Snapshot testing
- `tempfile` (3.12) - Temporary files for tests

## Future Enhancements

### Near-term
- [ ] Improve layout algorithm (better layer assignment)
- [ ] Support for deployment views
- [ ] Theme support
- [ ] Better text wrapping (consider actual font metrics)
- [ ] Orthogonal relationship routing

### Long-term
- [ ] PNG/PDF export
- [ ] Interactive HTML export
- [ ] Live reload/watch mode
- [ ] DSL language server (LSP)
- [ ] Visual editor integration

## Design Rationale

### Why Rust?
- **Performance**: Native speed for large diagrams
- **Safety**: Memory safety without GC
- **Portability**: Single binary, no runtime
- **Ecosystem**: Good crates for parsing, SVG, graphs

### Why Custom Parser?
- Better error messages with context
- More control over DSL extensions
- Simpler than learning parser combinator syntax
- Adequate performance for typical DSL files

### Why Not Use Existing Rendering?
- Web rendering uses JointJS (JavaScript library)
- Want standalone CLI tool
- SVG generation is straightforward
- More control over output format

### Why Layered Architecture?
- Clear separation of concerns
- Testable modules
- Easy to extend (new shapes, views, layouts)
- Standard pattern for compilers/generators
