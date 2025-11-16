## Example: How to use rustdocmd

Mark a section in your Rust code with the following pattern:

```rust
/// <chapter.md(1)> "main.rs"
/// # My Chapter
/// This is the documentation for this chapter.
/// </chapter.md>
```

After running `rustdocmd`, this will automatically generate a Markdown file `chapter.md` and include it in your mdBook.
The order in the table of contents is controlled by the number in parentheses `(1)`.

You can use as many such marker blocks as you like to structure your documentation.

If a block in the Rust code is changed, simply update the documentation by running `rustdocmd` again.

If a block is removed, the corresponding Markdown file and the entry in the table of contents will be automatically deleted.

## Generate README.md
To include a section in your `README.md`, use a marker like this:

```rust
/// <readme>
/// # My Section
/// This text will appear in the README.
/// </readme>
```

When you run `rustdocmd --generate-readme`, all such blocks are collected and written to `README.md` (previous content will be overwritten).
Without the `--generate-readme` flag, your `README.md` remains unchanged.

## Mirroring SUMMARY.md (mdBook)

By default, rustdocmd writes and maintains the table of contents for your mdBook at `mdbook/src/SUMMARY.md` (the location expected by mdBook).
For greater compatibility, this file can also be mirrored to `mdbook/SUMMARY.md` (project root), so both versions are always in sync.

- Default: Mirroring is enabled.
- You can disable it with the following CLI flag:

```
./target/release/rustdocmd --mirror-root-summary=false
```

If mirroring is disabled, only `mdbook/src/SUMMARY.md` will be updated; the file in the project root will remain untouched.