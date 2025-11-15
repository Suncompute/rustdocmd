use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn generates_markdown_and_summary() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let root = dir.path();
    // project structure
    let src_dir = root.join("src");
    let mdbook_src = root.join("mdbook").join("src");
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&mdbook_src)?;

    // sample Rust file with marker in rustdoc
    let sample_rs = r#"
    /// <intro.md(1)> "sample.rs"
    /// # Hello Integration
    ///
    /// Body
    /// </intro.md>
    "#;
    fs::write(src_dir.join("sample.rs"), sample_rs)?;

    // config file
    let toml = r#"
    [paths]
    source = "./src"
    target = "./mdbook/src"
    "#;
    fs::write(root.join("rustdocmd.toml"), toml)?;

    // run binary in temp root
    let mut cmd = Command::cargo_bin("rustdocmd")?;
    cmd.current_dir(&root);
    cmd.assert().success();

    // assert markdown file exists and contains content
    let md_path = mdbook_src.join("intro.md");
    let md = fs::read_to_string(&md_path)?;
    assert!(md.contains("# Hello Integration"));

    // assert SUMMARY.md exists and has entry
    let summary_path = mdbook_src.parent().unwrap().join("SUMMARY.md");
    let summary = fs::read_to_string(&summary_path)?;
    assert!(summary.contains("* [intro](intro.md)"));

    Ok(())
}
