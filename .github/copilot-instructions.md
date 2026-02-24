# GitHub Copilot Instructions

## What this repo is

A **Rust CLI tool** (`structurizr-svg`) that parses [Structurizr DSL](https://structurizr.com/dsl) files and generates SVG architecture diagrams (C4 model). It is a standalone binary with no Java or runtime dependencies.

## Project layout

```
src/
  main.rs          # CLI entry point (clap arg parsing)
  lib.rs           # generate_svgs() orchestration
  parser/
    dsl.rs         # Full DSL parser (tokenizer + recursive descent)
    tokens.rs      # TokenType enum + Token struct
    mod.rs
  model/
    mod.rs         # Workspace struct (elements HashMap, relationships Vec, views Vec, styles)
    element.rs     # Element, ElementType (Person/SoftwareSystem/Container/Component)
    relationship.rs
    style.rs       # ElementStyle, RelationshipStyle, Shape, Border, Styles
    view.rs        # View, ViewType, AutoLayout, Direction
  renderer/
    mod.rs         # SvgRenderer entry point
    svg.rs         # SVG document assembly
    shapes.rs      # All 18 Structurizr shapes rendered via svg crate
    layout.rs      # Auto-layout (hierarchical, petgraph-backed)
    text.rs        # Text wrapping helpers
  utils/

workspace.dsl      # Main workspace (uses !include for model/views/relationships)
model.dsl          # Element definitions (people, systems, containers)
relationships.dsl  # Relationship definitions (parsed inside model{} block)
views.dsl          # View definitions + styles
examples/
  basic.dsl        # Self-contained example (no !include)
tests/
  integration_tests.rs
```

## DSL workspace structure

The primary workspace is **split across four files** using `!include`:

```
workspace.dsl       → !include model.dsl
                    → !include relationships.dsl
                    → !include views.dsl
```

- `model.dsl` — defines people and software systems (with `group` blocks for trust boundaries)
- `relationships.dsl` — defines relationships between elements (referenced by variable name)
- `views.dsl` — defines views (`systemLandscape`, `systemContext`, `container`, `dynamic`, `filtered`) and styles

## Parser design

The parser is a hand-written recursive descent parser. Key design points:

### Preprocessing (`!include`)
`parse_file()` calls `preprocess_includes()` **before tokenizing**. It reads each `!include <file>` line and substitutes the file content inline. This means `!include` never reaches the tokenizer.

### Tokenizer rules
- `//` → line comment (skip to EOL)
- `#` followed by a hex digit (`[0-9a-fA-F]`) → **hex color String token** (e.g. `#08427b`)
- `#` followed by anything else → line comment
- `!` → skip to EOL (handles `!docs`, `!adrs`, `!identifiers` directives)
- `*` → `Identifier("*")` (wildcard for `include *`)
- `->` → `Arrow` token

### Key parsing logic

**Variable assignment detection** (`name = softwareSystem ...`):
In `parse_model_body`, when the current token is `Identifier`, peek one ahead. If `tokens[current+1]` is `Equals`, call `parse_element()` (which handles consuming `varName =`). Otherwise treat as a relationship (`source -> dest`).

**`parse_model_body`** is the shared helper for `model {}`, `group {}`, and `enterprise {}` blocks — all three are transparent wrappers that recurse into the same body-parsing logic. Variable bindings (`element_vars: HashMap<String, String>`) are passed through recursively, so container variable names defined inside a `softwareSystem {}` body are available for relationship parsing at the top level.

**Nested elements (containers/components)**:
`parse_element()` takes `&mut Workspace` and `&mut HashMap<String, String>`. When it parses children inside a `{ }` body, it adds them directly to the workspace and their var bindings to `element_vars`. Look-ahead (`tokens[current+2]`) checks if an `Identifier` at element body level is a variable-assigned `container`/`component`.

**`autoLayout`** direction argument is optional — defaults to `TopBottom` if the next token is not a String or Identifier.

**Unknown view blocks** (`dynamic`, `filtered`): `parse_views` has a `LeftBrace` arm that calls `skip_brace_block()` to skip any balanced `{...}` block that wasn't matched by a known view keyword.

### Element IDs
Elements get auto-generated IDs: `element_0`, `element_1`, ... (counter in `DslParser`). Relationships use `rel_0`, `rel_1`, ...

### Tags and styles
Elements automatically receive type-based default tags: `"Element"` + `"Person"` / `"Software System"` / `"Container"` / `"Component"`. The styles system matches tags to `ElementStyle` entries. Relationships similarly match `RelationshipStyle` by tag.

## Running / building

```bash
cargo build --release
cargo test
cargo run -- workspace.dsl -o ./output
cargo run -- examples/basic.dsl -o ./output
```

The binary name is `structurizr-svg`. The `-v` short flag is reserved for `--view`; use `--verbose` (long form only) for verbose logging.

## Known limitations / not yet supported

- `filtered` views — parsed but skipped (no output generated)
- `dynamic` views — parsed but skipped
- `!docs` / `!adrs` / `!identifiers` directives — silently ignored
- Themes — not supported
- Manual element positioning (only auto-layout)
- The `exclude` directive in views is parsed but not enforced

## Dependencies

| Crate | Purpose |
|-------|---------|
| `svg 0.18` | SVG document/element construction |
| `petgraph 0.6` | Graph algorithms for auto-layout |
| `clap 4.5` (derive) | CLI argument parsing |
| `thiserror` / `anyhow` | Error handling |
| `env_logger` / `log` | Logging |

## Test structure

`tests/integration_tests.rs` has three tests:
- `test_basic_dsl_parsing` — inline DSL string, checks SVG output
- `test_example_dsl` — parses `examples/basic.dsl`, checks all 4 view SVGs exist
- `test_view_filtering` — checks `--view` flag generates only the requested view
