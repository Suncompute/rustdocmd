## Beispiel: So verwendest du rustdocmd

Markiere im Rust-Code einen Bereich mit folgendem Muster:

```rust
/// <kapitel.md(1)> "main.rs"
/// # Mein Kapitel
/// Hier steht die Dokumentation für dieses Kapitel.
/// </kapitel.md>
```

Nach dem Ausführen von `rustdocmd` wird daraus automatisch eine Markdown-Datei `kapitel.md` erzeugt und in dein mdBook eingebunden.
Die Reihenfolge im Inhaltsverzeichnis steuerst du mit der Zahl in Klammern `(1)`.

Du kannst beliebig viele solcher Marker-Blöcke in deinem Code verwenden, um die Doku zu strukturieren.

Wenn ein Block im Rust Code geändert wird, aktualisiere einfach die Doku, indem du `rustdocmd` erneut ausführst.

Wird ein Block entfernt, so wird die entsprechende Markdown-Datei und der Eintrag im Inhaltsverzeichnis automatisch gelöscht.