//! Integration tests for the SVG CLI

use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_basic_dsl_parsing() {
    let dsl_content = r##"
workspace "Test" "Test workspace" {
    model {
        user = person "User"
        system = softwareSystem "System"
        user -> system "Uses"
    }
    
    views {
        systemContext system "SystemContext" {
            include *
            autolayout tb
        }
        
        styles {
            element "Person" {
                shape person
                background "#08427b"
            }
        }
    }
}
"##;

    let temp_dir = TempDir::new().unwrap();
    let dsl_path = temp_dir.path().join("test.dsl");
    fs::write(&dsl_path, dsl_content).unwrap();

    let output_dir = temp_dir.path().join("output");

    // Generate SVGs
    let result = structurizr_svg::generate_svgs(&dsl_path, &output_dir, None);
    assert!(result.is_ok(), "Failed to generate SVGs: {:?}", result.err());

    // Check that output was created
    assert!(output_dir.exists(), "Output directory was not created");

    // Check for SVG file
    let svg_file = output_dir.join("SystemContext.svg");
    assert!(svg_file.exists(), "SVG file was not generated");

    // Verify SVG content
    let svg_content = fs::read_to_string(&svg_file).unwrap();
    assert!(svg_content.contains("<svg"), "Invalid SVG content");
    assert!(svg_content.contains("User"), "Element name not found in SVG");
    assert!(svg_content.contains("System"), "Element name not found in SVG");
}

#[test]
fn test_example_dsl() {
    let dsl_path = Path::new("examples/basic.dsl");
    if !dsl_path.exists() {
        panic!("Example DSL file not found");
    }

    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("output");

    // Generate SVGs
    let result = structurizr_svg::generate_svgs(dsl_path, &output_dir, None);
    assert!(result.is_ok(), "Failed to generate SVGs: {:?}", result.err());

    // Check that all expected views were generated
    assert!(output_dir.join("SystemLandscape.svg").exists());
    assert!(output_dir.join("SystemContext.svg").exists());
    assert!(output_dir.join("Containers.svg").exists());
    assert!(output_dir.join("Components.svg").exists());
}

#[test]
fn test_view_filtering() {
    let dsl_path = Path::new("examples/basic.dsl");
    if !dsl_path.exists() {
        panic!("Example DSL file not found");
    }

    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("output");

    // Generate only SystemContext view
    let result = structurizr_svg::generate_svgs(dsl_path, &output_dir, Some("SystemContext"));
    assert!(result.is_ok(), "Failed to generate SVGs: {:?}", result.err());

    // Check that only SystemContext was generated
    assert!(output_dir.join("SystemContext.svg").exists());
    assert!(!output_dir.join("SystemLandscape.svg").exists());
    assert!(!output_dir.join("Containers.svg").exists());
}
