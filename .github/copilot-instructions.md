# rustdocmd – Copilot-Anweisungen

## Projektüberblick
`rustdocmd` ist ein Rust-CLI-Tool, das speziell markierte rustdoc-Kommentarbereiche aus `.rs`-Dateien extrahiert und als eigenständige Markdown-Dateien für die Integration in Dokumentationssysteme wie mdBook ausgibt.

## Architektur & Hauptkomponenten
- **Parser**: Extrahiert rustdoc-Kommentare aus `.rs`-Dateien (Regex oder AST)
- **Marker-Erkennung**: Findet spezielle Marker-Blöcke wie `<test.md(1)> "code.md"</test.md>`
        - Beispiel: `<kapitel.md(2)> "code.md"</kapitel.md>`
        - Die Zahl `(1)` gibt die Reihenfolge im `SUMMARY.md` an; fehlt sie, wird am Ende eingefügt.
- **Markdown-Writer**: Schreibt die extrahierten Inhalte als `.md`-Dateien in den `/src`-Ordner des mdBook-Projekts
- **CLI & Konfiguration**: Liest Pfade und Einstellungen aus CLI-Argumenten oder `rustdocmd.toml`

## Datenfluss
1. Lese Konfiguration aus `rustdocmd.toml` (Pfade zu `.rs`- und Ziel-`/src`-Ordner)
2. Scanne rekursiv Rust-Quellcode nach Markern
3. Extrahiere und transformiere rustdoc-Kommentare zu Markdown
4. Schreibe `.md`-Dateien ins Zielverzeichnis
5. Aktualisiere `SUMMARY.md` entsprechend der Marker-Reihenfolge

## Workflows & Befehle
**Initiales Setup:**
```bash
cargo new rustdocmd --bin
cargo add clap --features derive
cargo add anyhow
cargo add walkdir
```

**Build & Test:**
```bash
cargo build
cargo build --release
cargo test
cargo run -- --help
```

**Konfiguration:**
- Lege eine `rustdocmd.toml` an, z.B.:
    ```toml
    [paths]
    source = "./meinprojekt/src"
    target = "./mdbook/src"
    ```
- Das Tool liest beim Aufruf die Konfiguration ein.

## Integrationspunkte
- **mdBook**: Zielsystem für die generierten `.md`-Dateien; muss im Projekt installiert sein
- **File System**: Liest `.rs`-Dateien, schreibt `.md`-Dateien und aktualisiert `SUMMARY.md`
- **Rust Toolchain**: Standard-Build- und Test-Workflows

## Konventionen & Tests
- Standard Rust-Projektstruktur (`src/`, `tests/`)
- Nutze `cargo fmt` und `cargo clippy` für Codequalität
- Integrationstests mit temporären Verzeichnissen für Dateioperationen
- Zunächst nur mdBook unterstützen; Erweiterung auf andere Tools bei Bedarf
