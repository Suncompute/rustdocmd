use crate::parser::ReadmeBlock;
pub fn write_readme(readme_blocks: &[ReadmeBlock], readme_path: &Path, dry_run: bool) -> io::Result<()> {
    let mut content = String::new();
    for block in readme_blocks {
        // Remove <readme> and </readme> tags from the block content
        let cleaned = block.content
            .replace("<readme>", "")
            .replace("</readme>", "");
        content.push_str(cleaned.trim());
        content.push_str("\n\n");
    }
    if dry_run {
        println!("[dry-run] write {} ({} bytes)", readme_path.display(), content.len());
    } else {
        fs::write(readme_path, content.trim_end())?;
    }
    Ok(())
}
use crate::parser::MarkerBlock;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn title_from_filename(filename: &str) -> String {
    let stem = filename.trim_end_matches(".md");
    stem.split(|c: char| c == '-' || c == '_' || c == ' ')
        .filter(|s| !s.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn write_markdown_and_summary(
    blocks: &[MarkerBlock],
    target_dir: &Path,
    summary_path: &Path,
    dry_run: bool,
    mirror_root_summary: bool,
    generate_mermaid_svg: bool,
) -> io::Result<()> {
    // 1. Schreibe alle .md-Dateien
    for block in blocks {
        let md_path = target_dir.join(&block.target_md);
        // Remove <readme> and </readme> tags from the block content
        let cleaned = block.content
            .replace("<readme>", "")
            .replace("</readme>", "");
        if dry_run {
            println!(
                "[dry-run] write {} ({} bytes)",
                md_path.display(),
                cleaned.len()
            );
        } else {
            fs::write(&md_path, cleaned.trim())?;
        }
    }

    // Nach dem Schreiben: Mermaid-Blöcke verarbeiten (nur wenn aktiviert)
    if generate_mermaid_svg {
        process_mermaid_blocks(target_dir, dry_run)?;
    }

    // 2. SUMMARY.md einlesen (oder neu anlegen)
    let _summary = if summary_path.exists() {
        fs::read_to_string(summary_path)?
    } else {
        String::from("# Summary\n\n")
    };

    // 3. Map für Reihenfolge und Einträge bauen
    let mut entries: BTreeMap<usize, (&MarkerBlock, String)> = BTreeMap::new();
    let mut last_index = 0;
    for block in blocks {
        let idx = block.order.unwrap_or_else(|| {
            last_index += 1;
            1000 + last_index // große Zahl = ans Ende
        });
        let title = title_from_filename(&block.target_md);
        let entry = format!("* [{}]({})\n", title, block.target_md);
        entries.insert(idx, (block, entry));
    }

    // 4. Neue SUMMARY.md bauen
    let mut new_summary = String::from("# Summary\n\n");
    for (_idx, (_block, entry)) in &entries {
        new_summary.push_str(entry);
    }
    if dry_run {
        let entries_count = entries.len();
        println!(
            "[dry-run] write {} with {} entries",
            summary_path.display(),
            entries_count
        );
    } else {
        // Debug: Zeige, was in SUMMARY.md geschrieben wird
        eprintln!("[debug] writing {} with content:\n{}", summary_path.display(), new_summary);
        fs::write(summary_path, new_summary)?;
        // Optional: Auch die SUMMARY.md im mdBook-Root aktualisieren (Kompatibilität)
        if mirror_root_summary {
            if let Some(root_dir) = summary_path.parent().and_then(|p| p.parent()) {
                let root_summary = root_dir.join("SUMMARY.md");
                eprintln!("[debug] mirroring summary to {}", root_summary.display());
                let _ = fs::write(root_summary, fs::read_to_string(summary_path)?);
            }
        }
    }

    // 5. Entferne .md-Dateien und Einträge, die nicht mehr in blocks vorkommen
    let existing_files = match fs::read_dir(target_dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
            .map(|e| e.path())
            .collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };
    let valid_files: Vec<PathBuf> = blocks.iter().map(|b| target_dir.join(&b.target_md)).collect();
    for file in existing_files {
        if !valid_files.contains(&file) {
            if dry_run {
                println!("[dry-run] remove {}", file.display());
            } else {
                let _ = fs::remove_file(&file);
            }
        }
    }
    Ok(())
}

// Sucht in allen .md Dateien nach ```mermaid ... ``` Blöcken, erzeugt SVGs via mmdc und fügt Bildlinks ein.
fn process_mermaid_blocks(target_dir: &Path, dry_run: bool) -> io::Result<()> {
    let md_files: Vec<PathBuf> = match fs::read_dir(target_dir) {
        Ok(rd) => rd.filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|ext| ext == "md").unwrap_or(false))
            .collect(),
        Err(_) => Vec::new(),
    };

    // Prüfe, ob mmdc vorhanden ist
    let mmdc_available = Command::new("which")
        .arg("mmdc")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !mmdc_available {
        eprintln!("[mermaid] 'mmdc' nicht gefunden. Installiere mit: npm install -g @mermaid-js/mermaid-cli");
    }

    let mermaid_regex = regex::Regex::new(r"(?s)```mermaid\n(.*?)\n```\n?").unwrap();

    for file in md_files {
        let original = fs::read_to_string(&file)?;
        let mut modified = original.clone();
        let mut changed = false;
        let mut index = 0;
        for cap in mermaid_regex.captures_iter(&original) {
            index += 1;
            let diagram_code = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("diagram");
            let svg_name = format!("{}-mermaid-{}.svg", stem, index);
            let svg_path = target_dir.join(&svg_name);

            if mmdc_available {
                if dry_run {
                    println!("[dry-run][mermaid] generate {} from block {} in {}", svg_name, index, file.display());
                } else {
                    // Temporäre Quelldatei für mmdc
                    let tmp_src = target_dir.join(format!("__tmp_mermaid_{}.mmd", index));
                    fs::write(&tmp_src, diagram_code)?;
                    let status = Command::new("mmdc")
                        .arg("-i").arg(&tmp_src)
                        .arg("-o").arg(&svg_path)
                        .status();
                    match status {
                        Ok(s) if s.success() => {
                            fs::remove_file(&tmp_src).ok();
                            println!("[mermaid] SVG erzeugt: {}", svg_path.display());
                        }
                        Ok(s) => {
                            eprintln!("[mermaid] 'mmdc' fehlgeschlagen (exit {}): {}", s.code().unwrap_or(-1), file.display());
                        }
                        Err(e) => {
                            eprintln!("[mermaid] Fehler beim Ausführen von mmdc: {}", e);
                        }
                    }
                }
            }

            // Ersetze den Block durch Block + Bild-Link (wenn mmdc vorhanden), sonst nur Original belassen.
            let replacement = if mmdc_available {
                format!("```mermaid\n{}\n```\n\n![Mermaid Diagram]({})\n", diagram_code, svg_name)
            } else {
                format!("```mermaid\n{}\n```\n", diagram_code)
            };
            modified = modified.replacen(cap.get(0).unwrap().as_str(), &replacement, 1);
            changed = true;
        }
        if changed {
            if dry_run {
                println!("[dry-run][mermaid] update {} ({} -> {} bytes)", file.display(), original.len(), modified.len());
            } else {
                fs::write(&file, modified)?;
            }
        }
    }
    Ok(())
}
