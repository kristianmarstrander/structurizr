# Structurizr SVG CLI

A Rust-based command-line tool for generating high-quality SVG diagrams from Structurizr DSL files.

## Overview

This tool parses Structurizr DSL files and generates SVG diagrams, similar to what the Structurizr web interface produces. It's a standalone Rust project that can be used independently of the Java codebase.

## Features

- **Full DSL Support**: Parse workspace, model, views, and styles
- **All Shape Types**: Supports all 18 Structurizr shapes including Box, RoundedBox, Person, Robot, Cylinder, WebBrowser, MobileDevice, etc.
- **Complete Styling**: Element and relationship styles with colors, fonts, borders, opacity
- **Auto-Layout**: Automatic positioning using hierarchical layout algorithms
- **Multiple View Types**: SystemLandscape, SystemContext, Container, Component views
- **High Performance**: Fast native binary with no runtime dependencies

## Installation

### From Source

```bash
cd structurizr-svg-cli
cargo build --release
```

The binary will be available at `target/release/structurizr-svg`.

### Using Cargo

```bash
cargo install --path .
```

## Quick Start

```bash
# Generate SVGs from a DSL file
./target/release/structurizr-svg examples/basic.dsl

# Specify output directory
./target/release/structurizr-svg examples/basic.dsl -o ./diagrams

# Generate only a specific view
./target/release/structurizr-svg examples/basic.dsl -v SystemContext

# Use custom canvas size
./target/release/structurizr-svg examples/basic.dsl -w 3000 -H 2000

# Enable verbose logging
./target/release/structurizr-svg examples/basic.dsl -v
```

## CLI Reference

```
structurizr-svg [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to Structurizr DSL file (.dsl)

Options:
  -o, --output <DIR>       Output directory for SVG files [default: ./output]
  -v, --view <KEY>         Render only the specified view (by key)
  -w, --width <WIDTH>      Canvas width [default: 2000]
  -H, --height <HEIGHT>    Canvas height [default: 1500]
  --dark-mode              Use dark mode colors
  --no-metadata            Hide element type labels
  --no-description         Hide element descriptions
  --verbose                Enable verbose logging
  --help                   Print help
  --version               Print version
```

## Supported DSL Features

### Workspace

```dsl
workspace "Name" "Description" {
    // workspace content
}
```

### Model Elements

```dsl
model {
    person "User" "Description"
    softwareSystem "System" "Description" {
        container "Container" "Description" "Technology" {
            component "Component" "Description" "Technology"
        }
    }
}
```

### Relationships

```dsl
element1 -> element2 "Description" "Technology"
```

### Views

```dsl
views {
    systemLandscape "Key" "Title" {
        include *
        autolayout tb
    }
    
    systemContext system "Key" "Title" {
        include *
        autolayout lr
    }
    
    container system "Key" "Title" {
        include *
        autolayout rl
    }
    
    component container "Key" "Title" {
        include *
        autolayout bt
    }
}
```

### Styles

```dsl
styles {
    element "Tag" {
        shape roundedbox
        background "#438dd5"
        color "#ffffff"
        stroke "#333333"
        strokeWidth 2
        fontSize 24
        width 450
        height 300
        border solid
        opacity 100
    }
    
    relationship "Tag" {
        color "#707070"
        thickness 2
        style solid
    }
}
```

## Supported Shapes

The tool supports all Structurizr shapes:

- **Box** - Simple rectangle
- **RoundedBox** - Rectangle with rounded corners (default)
- **Circle** - Perfect circle
- **Ellipse** - Ellipse shape
- **Hexagon** - Six-sided polygon
- **Diamond** - Rotated square
- **Cylinder** - Database-style cylinder
- **Pipe** - Queue-style horizontal cylinder
- **Person** - Stylized person icon
- **Robot** - Robot icon variant
- **Folder** - Folder shape with tab
- **Component** - UML component shape
- **WebBrowser** - Browser window frame
- **Window** - Desktop window frame
- **Terminal** - Terminal window
- **Shell** - Command shell
- **MobileDevicePortrait** - Mobile device (portrait)
- **MobileDeviceLandscape** - Mobile device (landscape)

## Auto-Layout Directions

- `tb` or `topbottom` - Top to bottom (default)
- `bt` or `bottomtop` - Bottom to top
- `lr` or `leftright` - Left to right
- `rl` or `rightleft` - Right to left

## Example DSL

See `examples/basic.dsl` for a complete example including:
- Multiple element types (Person, SoftwareSystem, Container, Component)
- Nested containers and components
- Relationships with descriptions and technologies
- Multiple view types
- Comprehensive styling

## Comparison with Structurizr Web

This tool aims to produce SVG output similar to the Structurizr web interface:

- ✅ All shape types supported
- ✅ Full styling support (colors, fonts, borders)
- ✅ Auto-layout algorithms
- ✅ Multiple view types
- ✅ Text wrapping and formatting
- ⚠️ Layout algorithm may differ slightly from web version
- ❌ Themes not yet supported
- ❌ Documentation/ADR views not supported
- ❌ Interactive features (web-only)

## Troubleshooting

### Parser Errors

If you encounter parser errors:
- Ensure your DSL file uses proper syntax
- Check that all braces are balanced
- Verify that strings are properly quoted
- Use `--verbose` flag for detailed logging

### Layout Issues

If element positions look incorrect:
- Try different auto-layout directions (`tb`, `lr`, etc.)
- Adjust canvas size with `-w` and `-H` options
- Check that all relationships are valid

### Rendering Issues

If shapes or text don't render correctly:
- Verify your SVG viewer supports SVG 1.1
- Check that colors are valid hex codes
- Ensure font-size values are reasonable

## Architecture

For detailed architecture information, see [ARCHITECTURE.md](ARCHITECTURE.md).

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Submit a pull request

## License

Apache License 2.0 - See LICENSE file for details

## Acknowledgments

- Based on the Structurizr DSL specification
- Shape rendering inspired by structurizr-diagram.js
- Layout algorithms inspired by dagre and graphviz
