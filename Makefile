# Einfaches Makefile für rustdocmd

all: rustdocmd

rustdocmd:
	cargo build --release

clean:
	cargo clean

run:
	cargo run -- --help

test:
	cargo test
