/// <introducing.md(1)> "main.rs"
/// <readme>
/// rustdocmd Test
///
/// With this tool, you can write comprehensive, versioned documentation directly in your Rust code using rustdoc comments (`///` or `//!`).
///
/// Specially marked sections are automatically extracted and output as standalone Markdown files.
///
/// This way, your code and comments become a complete, always up-to-date documentation that can be seamlessly and automatically integrated into systems like mdBook—ensuring your project documentation always matches the current state of your code.
///
/// The tool is still in a prototype stage and is continuously being developed.
///
/// 👉 [Buy Me a Coffee](https://www.buymeacoffee.com/suncompute)
///
/// </readme>
/// </introducing.md>
/// <install.md(2)> "main.rs"
///
/// ## Installation Guide for rustdocmd
///
/// 1. Clone the repository:
///    ```sh
///    git clone https://github.com/Suncompute/rustdocmd.git
///    cd rustdocmd/rustdocmd
///    ```
/// 2. Install dependencies and build the release binary:
///    ```sh
///    cargo build --release
///    ```
/// 3. Create a `rustdocmd.toml` configuration file (if not already present):
///    ```toml
///    [paths]
///    source = "./src"
///    target = "./mdbook/src"
///    ```
///
/// 4. (Optional) Install mdBook if not already installed:
///    ```sh
///    cargo install mdbook
///    ```
/// 5. Run the tool:
///    ```sh
///    ./target/release/rustdocmd
///    ```
/// 6. View the documentation locally:
///    ```sh
///    cd mdbook
///    mdbook serve
///    # open http://localhost:3000 in your browser
///    ```
/// </install.md>
/// <example.md(3)> "main.rs"
///
/// ## Example: How to use rustdocmd
///
/// Mark a section in your Rust code with the following pattern:
///
/// ```rust
/// /// <chapter.md(1)> "main.rs"
/// /// # My Chapter
/// /// This is the documentation for this chapter.
/// /// </chapter.md>
/// ```
///
/// After running `rustdocmd`, this will automatically generate a Markdown file `chapter.md` and include it in your mdBook.
/// The order in the table of contents is controlled by the number in parentheses `(1)`.
///
/// You can use as many such marker blocks as you like to structure your documentation.
///
/// If a block in the Rust code is changed, simply update the documentation by running `rustdocmd` again.
///
/// If a block is removed, the corresponding Markdown file and the entry in the table of contents will be automatically deleted.
///
/// ## Generate README.md
/// To include a section in your `README.md`, use a marker like this:
///
/// ```rust
/// /// <readme>
/// /// # My Section
/// /// This text will appear in the README.
/// /// </readme>
/// ```
///
/// When you run `rustdocmd --generate-readme`, all such blocks are collected and written to `README.md` (previous content will be overwritten).
/// Without the `--generate-readme` flag, your `README.md` remains unchanged.
///
/// ## Mirroring SUMMARY.md (mdBook)
///
/// By default, rustdocmd writes and maintains the table of contents for your mdBook at `mdbook/src/SUMMARY.md` (the location expected by mdBook).
/// For greater compatibility, this file can also be mirrored to `mdbook/SUMMARY.md` (project root), so both versions are always in sync.
///
/// - Default: Mirroring is enabled.
/// - You can disable it with the following CLI flag:
///
/// ```
/// ./target/release/rustdocmd --mirror-root-summary=false
/// ```
///
/// If mirroring is disabled, only `mdbook/src/SUMMARY.md` will be updated; the file in the project root will remain untouched.
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