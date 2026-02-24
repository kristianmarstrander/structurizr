use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Structurizr SVG CLI - Generate SVG diagrams from Structurizr DSL files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to Structurizr DSL file (.dsl)
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output directory for SVG files
    #[arg(short, long, default_value = "./output")]
    output: PathBuf,

    /// Render only the specified view (by key)
    #[arg(short, long)]
    view: Option<String>,

    /// Canvas width
    #[arg(short = 'w', long, default_value = "2000")]
    width: u32,

    /// Canvas height
    #[arg(short = 'H', long, default_value = "1500")]
    height: u32,

    /// Use dark mode colors
    #[arg(long)]
    dark_mode: bool,

    /// Hide element type labels
    #[arg(long)]
    no_metadata: bool,

    /// Hide element descriptions
    #[arg(long)]
    no_description: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    log::info!("Structurizr SVG CLI v{}", env!("CARGO_PKG_VERSION"));
    log::info!("Input: {:?}", args.input);
    log::info!("Output: {:?}", args.output);

    // Validate input file exists
    if !args.input.exists() {
        anyhow::bail!("Input file does not exist: {:?}", args.input);
    }

    // Create output directory
    std::fs::create_dir_all(&args.output)?;

    // Generate SVGs
    structurizr_svg::generate_svgs(
        &args.input,
        &args.output,
        args.view.as_deref(),
    )?;

    log::info!("Done!");
    Ok(())
}
