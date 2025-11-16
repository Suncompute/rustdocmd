## Installationsanleitung für rustdocmd

1. Repository klonen:
```sh
git clone https://github.com/Suncompute/rustdocmd.git
cd rustdocmd/rustdocmd
```
2. Abhängigkeiten installieren und Release-Binary bauen:
```sh
cargo build --release
```
3. Konfigurationsdatei `rustdocmd.toml` anlegen (falls noch nicht vorhanden):
```toml
[paths]
source = "./src"
target = "./mdbook/src"
```

4. (Optional) mdBook installieren, falls noch nicht vorhanden:
```sh
cargo install mdbook
```
5. Tool ausführen:
```sh
./target/release/rustdocmd
```
6. Dokumentation lokal anzeigen:
```sh
cd mdbook
mdbook serve
# öffne http://localhost:3000 im Browser
```