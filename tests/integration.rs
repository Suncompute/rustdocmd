// Integrationstest-Platzhalter
// Hier können Tests für das End-to-End-Verhalten von rustdocmd geschrieben werden

#[test]
fn test_config_load() {
    let config = rustdocmd::config::RustdocmdConfig::from_file("rustdocmd.toml");
    assert!(config.is_ok());
}
