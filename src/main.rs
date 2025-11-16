/// <example.md(3)> "main.rs"
///
/// ## Beispiel: So verwendest du rustdocmd
///
/// Markiere im Rust-Code einen Bereich mit folgendem Muster:
///
/// ```rust
/// /// <kapitel.md(1)> "main.rs"
/// /// # Mein Kapitel
/// /// Hier steht die Dokumentation für dieses Kapitel.
/// /// </kapitel.md>
/// ```
///
/// Nach dem Ausführen von `rustdocmd` wird daraus automatisch eine Markdown-Datei `kapitel.md` erzeugt und in dein mdBook eingebunden.
/// Die Reihenfolge im Inhaltsverzeichnis steuerst du mit der Zahl in Klammern `(1)`.
///
/// Du kannst beliebig viele solcher Marker-Blöcke in deinem Code verwenden, um die Doku zu strukturieren.
///
/// </example.md>

/// <introducing.md(1)> "main.rs"
/// # Willkommen zu rustdocmd
/// 
/// Mit diesem Tool kannst du direkt im Rust-Code mit rustdoc-Kommentaren (`///` oder `//!`) umfangreiche Dokumentation verfassen.
/// Speziell markierte Bereiche werden automatisch extrahiert und als eigenständige Markdown-Dateien ausgegeben.
/// So entsteht aus deinem Code und den Kommentaren eine vollständige, versionierte Dokumentation, die sich nahtlos mit mdBook aufbereiten und veröffentlichen lässt.
/// 
/// - Marker-Syntax: <datei.md(reihenfolge)> "quelle"
/// - Automatische Integration in mdBook
/// 
/// </introducing.md>
/// <install.md(2)> "main.rs"
///
/// ## Installationsanleitung für rustdocmd
///
/// 1. Repository klonen:
///    ```sh
///    git clone https://github.com/Suncompute/rustdocmd.git
///    cd rustdocmd/rustdocmd
///    ```
/// 2. Abhängigkeiten installieren und Release-Binary bauen:
///    ```sh
///    cargo build --release
///    ```
/// 3. Konfigurationsdatei `rustdocmd.toml` anlegen (falls noch nicht vorhanden):
///    ```toml
///    [paths]
///    source = "./src"
///    target = "./mdbook/src"
///    ```
///
/// 4. (Optional) mdBook installieren, falls noch nicht vorhanden:
///    ```sh
///    cargo install mdbook
///    ```
/// 5. Tool ausführen:
///    ```sh
///    ./target/release/rustdocmd
///    ```
/// 6. Dokumentation lokal anzeigen:
///    ```sh
///    cd mdbook
///    mdbook serve
///    # öffne http://localhost:3000 im Browser
///    ```
/// </install.md>
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
    /// Zusätzlich die SUMMARY.md im mdBook-Root spiegeln (mdbook/SUMMARY.md)
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    mirror_root_summary: bool,
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
    writer::write_markdown_and_summary(&all_blocks, Path::new(target_dir), &summary_path, cli.dry_run, cli.mirror_root_summary)?;
    // Fallback: Falls SUMMARY.md im src/-Ordner fehlt, aber im mdbook-Root existiert, kopiere sie herüber
    if !cli.dry_run {
        let root_summary = Path::new(target_dir).parent().map(|p| p.join("SUMMARY.md"));
        if !summary_path.exists() {
            if let Some(root) = root_summary {
                if root.exists() {
                    fs::copy(&root, &summary_path)?;
                }
            }
        }
    }
    println!("{} Marker-Blöcke verarbeitet.", all_blocks.len());
    Ok(())
}