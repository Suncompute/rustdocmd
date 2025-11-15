use crate::parser::MarkerBlock;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Schreibt alle Marker-Blöcke als .md-Dateien ins Zielverzeichnis und aktualisiert die SUMMARY.md
pub fn write_markdown_and_summary(
    blocks: &[MarkerBlock],
    target_dir: &Path,
    summary_path: &Path,
) -> io::Result<()> {
    // 1. Schreibe alle .md-Dateien
    for block in blocks {
        let md_path = target_dir.join(&block.target_md);
        fs::write(&md_path, &block.content)?;
    }

    // 2. SUMMARY.md einlesen (oder neu anlegen)
    let mut summary = if summary_path.exists() {
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
        let entry = format!("* [{}]({})\n", block.target_md.trim_end_matches(".md"), block.target_md);
        entries.insert(idx, (block, entry));
    }

    // 4. Neue SUMMARY.md bauen
    let mut new_summary = String::from("# Summary\n\n");
    for (_idx, (_block, entry)) in &entries {
        new_summary.push_str(entry);
    }
    fs::write(summary_path, new_summary)?;

    // 5. Entferne .md-Dateien und Einträge, die nicht mehr in blocks vorkommen
    let existing_files = fs::read_dir(target_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .map(|e| e.path())
        .collect::<Vec<_>>();
    let valid_files: Vec<PathBuf> = blocks.iter().map(|b| target_dir.join(&b.target_md)).collect();
    for file in existing_files {
        if !valid_files.contains(&file) {
            let _ = fs::remove_file(&file);
        }
    }
    Ok(())
}
