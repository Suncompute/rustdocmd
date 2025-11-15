/// <introducing.md(1)> "main.rs"
/// # Willkommen zu rustdocmd
/// 
/// Dieses Projekt extrahiert spezielle rustdoc-Kommentare und wandelt sie in Markdown für mdBook um.
/// 
/// - Marker-Syntax: <datei.md(reihenfolge)> "quelle"
/// - Automatische Integration in mdBook
/// 
/// Viel Spaß beim Testen!
/// </introducing.md>
mod parser;
mod config;
mod writer;

use anyhow::Result;
use clap::Parser;
use config::RustdocmdConfig;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value = "rustdocmd.toml")]
    config: String,
    /// Nur anzeigen, was geschrieben/entfernt würde (keine Änderungen)
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = RustdocmdConfig::from_file(&cli.config)?;
    let source_dir = &config.paths.source;
    let target_dir = &config.paths.target;
    // mdBook erwartet die SUMMARY.md im src/-Ordner
    let summary_path = Path::new(target_dir).join("SUMMARY.md");

    // Zielverzeichnis sicherstellen (sofern nicht dry-run)
    if !cli.dry_run {
        fs::create_dir_all(target_dir)?;
    }

    // Alle .rs-Dateien rekursiv einlesen und Marker extrahieren
    let mut all_blocks = Vec::new();
    let mut parsed_files = Vec::new();
    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map(|ext| ext == "rs").unwrap_or(false) {
            let content = fs::read_to_string(entry.path())?;
            // Debug: Zeige extrahierte Rustdoc-Kommentare
            let doc_comments = parser::extract_rustdoc_comments(&content);
            println!("\n[Rustdoc-Kommentare aus {}]:\n{}", entry.path().display(), doc_comments);
            let blocks = parser::extract_marker_blocks(&content);
            if !blocks.is_empty() {
                println!("Marker gefunden in: {}", entry.path().display());
                for (i, block) in blocks.iter().enumerate() {
                    println!("  Block {}: target_md={}, order={:?}, source_ref='{}', content='{}'", i+1, block.target_md, block.order, block.source_ref, block.content);
                }
            }
            all_blocks.extend(blocks);
            parsed_files.push(entry.path().display().to_string());
        }
    }
    println!("Geparste Dateien:");
    for file in &parsed_files {
        println!("- {}", file);
    }

    // Markdown-Dateien und SUMMARY.md schreiben
    writer::write_markdown_and_summary(&all_blocks, Path::new(target_dir), &summary_path, cli.dry_run)?;
    println!("{} Marker-Blöcke verarbeitet.", all_blocks.len());
    Ok(())
}