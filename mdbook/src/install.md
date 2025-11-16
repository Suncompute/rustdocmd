## Installation Guide for rustdocmd

1. Clone the repository:
```sh
git clone https://github.com/Suncompute/rustdocmd.git
cd rustdocmd/rustdocmd
```
2. Install dependencies and build the release binary:
```sh
cargo build --release
```
3. Create a `rustdocmd.toml` configuration file (if not already present):
```toml
[paths]
source = "./src"
target = "./mdbook/src"
```

4. (Optional) Install mdBook if not already installed:
```sh
cargo install mdbook
```
5. Run the tool:
```sh
./target/release/rustdocmd
```
6. View the documentation locally:
```sh
cd mdbook
mdbook serve
# open http://localhost:3000 in your browser
```