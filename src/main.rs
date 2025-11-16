
/// <introducing.md(1)> "main.rs"
/// <readme>
/// # rustdocmd
/// 
/// Mit diesem Tool kannst du direkt im Rust-Code mit rustdoc-Kommentaren (`///` oder `//!`) umfangreiche Dokumentation verfassen.
/// Speziell markierte Bereiche werden automatisch extrahiert und als eigenständige Markdown-Dateien ausgegeben.
/// So entsteht aus deinem Code und den Kommentaren eine vollständige, versionierte Dokumentation, die sich nahtlos mit mdBook aufbereiten und veröffentlichen lässt.
/// </readme>
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
/// Wenn ein Block im Rust Code geändert wird, aktualisiere einfach die Doku, indem du `rustdocmd` erneut ausführst.
///
/// Wird ein Block entfernt, so wird die entsprechende Markdown-Datei und der Eintrag im Inhaltsverzeichnis automatisch gelöscht.
///
/// ## README.md generieren
/// Um einen Abschnitt in deine `README.md` aufzunehmen, verwende einen Marker wie diesen:
///
/// ```rust
/// /// <readme>
/// /// # Mein Abschnitt
/// /// Dieser Text erscheint in der README.
/// /// </readme>
/// ```
///
/// Wenn du `rustdocmd --generate-readme` ausführst, werden alle solchen Blöcke gesammelt und in die `README.md` geschrieben (vorheriger Inhalt wird überschrieben).
/// Ohne das Flag `--generate-readme` bleibt deine `README.md` unverändert.
///
/// ## Spiegelung der SUMMARY.md (mdBook)
///
/// Standardmäßig schreibt und pflegt rustdocmd die Inhaltsübersicht deines mdBook unter `mdbook/src/SUMMARY.md` (das ist der Ort, den mdBook erwartet).
/// Für mehr Kompatibilität kann diese Datei zusätzlich nach `mdbook/SUMMARY.md` (Projekt-Root) gespiegelt werden, sodass beide Versionen immer synchron sind.
///
/// - Standard: Spiegelung ist aktiviert.
/// - Du kannst sie mit folgendem CLI-Flag deaktivieren:
///
/// ```
/// ./target/release/rustdocmd --mirror-root-summary=false
/// ```
///
/// Ist die Spiegelung deaktiviert, wird nur `mdbook/src/SUMMARY.md` aktualisiert; die Datei im Projekt-Root bleibt unberührt.
/// </example.md>
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
    /// README.md aus <readme>-Blöcken generieren (überschreibt README.md!)
    #[arg(long, default_value_t = false)]
    generate_readme: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = RustdocmdConfig::from_file(&cli.config)?;
    let source_dir = &config.paths.source;
    let target_dir = &config.paths.target;
    let summary_path = Path::new(target_dir).join("SUMMARY.md");

    if !cli.dry_run {
        fs::create_dir_all(target_dir)?;
    }

    let mut all_blocks = Vec::new();
    let mut parsed_files = Vec::new();
    let mut all_readme_blocks = Vec::new();
    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map(|ext| ext == "rs").unwrap_or(false) {
            let content = fs::read_to_string(entry.path())?;
            let doc_comments = parser::extract_rustdoc_comments(&content);
            println!("\n[Rustdoc-Kommentare aus {}]:\n{}", entry.path().display(), doc_comments);
            let blocks = parser::extract_marker_blocks(&content);
            let readme_blocks = parser::extract_readme_blocks(&content);
            if !blocks.is_empty() {
                println!("Marker gefunden in: {}", entry.path().display());
                for (i, block) in blocks.iter().enumerate() {
                    println!("  Block {}: target_md={}, order={:?}, source_ref='{}', content='{}'", i+1, block.target_md, block.order, block.source_ref, block.content);
                }
            }
            all_blocks.extend(blocks);
            all_readme_blocks.extend(readme_blocks);
            parsed_files.push(entry.path().display().to_string());
        }
    }
    println!("Geparste Dateien:");
    for file in &parsed_files {
        println!("- {}", file);
    }

    writer::write_markdown_and_summary(&all_blocks, Path::new(target_dir), &summary_path, cli.dry_run, cli.mirror_root_summary)?;
    if cli.generate_readme {
        let readme_path = Path::new("README.md");
        writer::write_readme(&all_readme_blocks, readme_path, cli.dry_run)?;
        println!("README.md wurde aus {} Block(s) generiert.", all_readme_blocks.len());
    }
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