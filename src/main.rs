mod parser;
mod config;
mod writer;

use anyhow::Result;
use clap::Parser;
use config::RustdocmdConfig;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value = "rustdocmd.toml")]
    config: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = RustdocmdConfig::from_file(&cli.config)?;
    // TODO: Scan source, parse markers, write markdown, update SUMMARY.md
    println!("Konfiguration geladen: {:?}", config);
    Ok(())
}
