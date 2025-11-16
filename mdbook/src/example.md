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

## Spiegelung der SUMMARY.md (mdBook)

Standardmäßig schreibt und pflegt rustdocmd die Inhaltsübersicht deines mdBook unter `mdbook/src/SUMMARY.md` (das ist der Ort, den mdBook erwartet).
Für mehr Kompatibilität kann diese Datei zusätzlich nach `mdbook/SUMMARY.md` (Projekt-Root) gespiegelt werden, sodass beide Versionen immer synchron sind.

- Standard: Spiegelung ist aktiviert.
- Du kannst sie mit folgendem CLI-Flag deaktivieren:

```
./target/release/rustdocmd --mirror-root-summary=false
```

Ist die Spiegelung deaktiviert, wird nur `mdbook/src/SUMMARY.md` aktualisiert; die Datei im Projekt-Root bleibt unberührt.

## README.md generieren
Um einen Abschnitt in deine `README.md` aufzunehmen, verwende einen Marker wie diesen:

```rust
/// <readme>
/// # Mein Abschnitt
/// Dieser Text erscheint in der README.
/// </readme>
```

Wenn du `rustdocmd --generate-readme` ausführst, werden alle solchen Blöcke gesammelt und in die `README.md` geschrieben (vorheriger Inhalt wird überschrieben).
Ohne das Flag `--generate-readme` bleibt deine `README.md` unverändert.