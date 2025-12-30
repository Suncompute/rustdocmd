use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct RustdocmdConfig {
    pub paths: Paths,
}

#[derive(Debug, Deserialize)]
pub struct Paths {
    pub source: String,
    pub target: String,
}

impl RustdocmdConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: RustdocmdConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
