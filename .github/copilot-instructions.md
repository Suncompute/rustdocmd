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
6. Entferne `.md`-Dateien und zugehörige Einträge aus der `SUMMARY.md`, wenn die Marker im Quellcode nicht mehr vorhanden sind

**Initiales Setup:**
```bash
cargo new rustdocmd --bin
cargo add clap --features derive
cargo add anyhow
cargo add walkdir
```

## mdBook-Installation & Nutzung

**mdBook installieren:**
```bash
cargo install mdbook
```

**mdBook-Projekt initialisieren:**
```bash
mdbook init mdbook --title "rustdocmd Dokumentation"
```
Das erzeugt im Ordner `mdbook/` die Grundstruktur für die Dokumentation. Die von rustdocmd generierten `.md`-Dateien werden in `mdbook/src/` abgelegt.

**mdBook lokal bauen und anzeigen:**
```bash
cd mdbook
mdbook serve
```
Danach ist die Dokumentation unter http://localhost:3000 erreichbar.

**Automatische mdBook-Installation:**
Es ist möglich, beim ersten Start von rustdocmd zu prüfen, ob `mdbook` installiert ist, und ggf. mit `cargo install mdbook` nachzuinstallieren (z.B. via `std::process::Command`).
Empfohlen wird aber, die Installation als Schritt in der Projektanleitung zu dokumentieren und nicht automatisch im Binary auszuführen, da dies Rechte und Toolchain-Voraussetzungen voraussetzt.
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
