# rustdocmd
rustdocmd is a simple tool that extracts specially marked rustdoc comment regions from Rust source files and writes them as standalone Markdown files. This enables seamless, automated integration of code documentation into systems like mdBook, keeping your project docs always in sync with your codebase.

## Summary mirroring (mdBook)

By default, rustdocmd writes and maintains your mdBook table of contents at `mdbook/src/SUMMARY.md` (the location mdBook expects). For convenience and compatibility with some workflows, it can also mirror that file to `mdbook/SUMMARY.md` (project root) so both locations stay in sync.

- Default: mirroring is enabled.
- You can disable it with the CLI flag:

```
./target/release/rustdocmd --mirror-root-summary=false
```

When disabled, only `mdbook/src/SUMMARY.md` is updated; the root `mdbook/SUMMARY.md` is left untouched.
