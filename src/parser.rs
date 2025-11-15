use regex::Regex;

#[derive(Debug, Clone)]
pub struct MarkerBlock {
    pub target_md: String,      // z.B. "test.md"
    pub order: Option<usize>,  // z.B. 1
    pub source_ref: String,    // z.B. "code.md" oder leer
    pub content: String,       // extrahierter Inhalt
}

/// Extrahiert alle Marker-Blöcke aus Rustdoc-Kommentaren (blockweise, Zeilenumbrüche erlaubt)
pub fn extract_marker_blocks(source: &str) -> Vec<MarkerBlock> {
    let doc = extract_rustdoc_comments(source);
    let lines: Vec<&str> = doc.lines().collect();
    // Öffnende Zeile: <file.md(1)> optional gefolgt von "source_ref"
    let re_open = Regex::new(r#"^\s*<([\w\-.]+)(?:\((\d+)\))?>\s*(?:\"([^\"]+)\")?\s*$"#).unwrap();
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let Some(cap) = re_open.captures(lines[i]) {
            let tag = cap[1].to_string();
            let order = cap.get(2).and_then(|m| m.as_str().parse::<usize>().ok());
            let source_ref = cap.get(3).map(|m| m.as_str().to_string()).unwrap_or_default();
            // Suche nach passendem schließenden Tag ab der nächsten Zeile
            let re_close = Regex::new(&format!(r#"^\s*</{}>\s*$"#, regex::escape(&tag))).unwrap();
            let mut j = i + 1;
            while j < lines.len() && !re_close.is_match(lines[j]) {
                j += 1;
            }
            if j < lines.len() { // schließendes Tag gefunden
                let content = lines[i+1..j].join("\n").trim().to_string();
                blocks.push(MarkerBlock { target_md: tag, order, source_ref, content });
                i = j + 1; // weiter nach dem schließenden Tag
                continue;
            } else {
                // kein schließendes Tag gefunden -> ignoriere diesen offenen Marker
            }
        }
        i += 1;
    }
    blocks
}

pub fn extract_rustdoc_comments(source: &str) -> String {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            if let Some(rest) = line.strip_prefix("///") {
                Some(rest.trim())
            } else if let Some(rest) = line.strip_prefix("//!") {
                Some(rest.trim())
            } else { None }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
