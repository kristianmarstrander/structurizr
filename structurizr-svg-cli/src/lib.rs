//! Structurizr SVG CLI Library
//!
//! A Rust library for parsing Structurizr DSL files and generating SVG diagrams.

pub mod model;
pub mod parser;
pub mod renderer;
pub mod utils;

pub use model::{Element, ElementType, Relationship, View, ViewType, Workspace};
pub use parser::DslParser;
pub use renderer::SvgRenderer;

use anyhow::Result;
use std::path::Path;

/// Parse a DSL file and generate SVG diagrams
pub fn generate_svgs<P: AsRef<Path>>(
    dsl_path: P,
    output_dir: P,
    view_key: Option<&str>,
) -> Result<()> {
    // Parse DSL file
    let mut parser = DslParser::new();
    let workspace = parser.parse_file(dsl_path)?;

    // Generate SVGs for views
    let renderer = SvgRenderer::new();
    
    for view in &workspace.views {
        if let Some(key) = view_key {
            if view.key != key {
                continue;
            }
        }
        
        let svg_content = renderer.render_view(&workspace, view)?;
        let output_path = output_dir.as_ref().join(format!("{}.svg", view.key));
        std::fs::write(&output_path, svg_content)?;
        log::info!("Generated: {:?}", output_path);
    }

    Ok(())
}
