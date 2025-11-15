use regex::Regex;

#[derive(Debug, Clone)]
pub struct MarkerBlock {
    pub target_md: String,      // z.B. "test.md"
    pub order: Option<usize>,  // z.B. 1
    pub source_ref: String,    // z.B. "code.md"
    pub content: String,       // extrahierter Inhalt
}

/// Extrahiert alle Marker-Blöcke aus einem gegebenen Rust-Quelltext
pub fn extract_marker_blocks(source: &str) -> Vec<MarkerBlock> {
    // Regex für Marker-Start: <dateiname.md(zahl)?> "quelle.md">
    let re = Regex::new(r#"<([\w\-.]+)(?:\((\d+)\))?>\s*\"([\w\-.]+)\"\s*>([\s\S]*?)</\1>"#).unwrap();
    let mut blocks = Vec::new();
    for cap in re.captures_iter(source) {
        let target_md = cap[1].to_string();
        let order = cap.get(2).map(|m| m.as_str().parse::<usize>().ok()).flatten();
        let source_ref = cap[3].to_string();
        let content = cap[4].trim().to_string();
        blocks.push(MarkerBlock { target_md, order, source_ref, content });
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_marker_blocks() {
        let src = r#"
        /// <kapitel.md(2)> "code.md"
        /// # Überschrift
        /// Inhalt
        /// </kapitel.md>
        /// <test.md> "quelle.rs"
        /// Noch ein Block
        /// </test.md>
        "#;
        let blocks = extract_marker_blocks(src);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].target_md, "kapitel.md");
        assert_eq!(blocks[0].order, Some(2));
        assert_eq!(blocks[0].source_ref, "code.md");
        assert!(blocks[0].content.contains("Überschrift"));
        assert_eq!(blocks[1].target_md, "test.md");
        assert_eq!(blocks[1].order, None);
        assert_eq!(blocks[1].source_ref, "quelle.rs");
    }
}
