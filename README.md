rustdocmd Test

With this tool, you can write comprehensive, versioned documentation directly in your Rust code using rustdoc comments (`///` or `//!`).

Specially marked sections are automatically extracted and output as standalone Markdown files.

This way, your code and comments become a complete, always up-to-date documentation that can be seamlessly and automatically integrated into systems like mdBook—ensuring your project documentation always matches the current state of your code.

The tool is still in a prototype stage and is continuously being developed.

👉 [Buy Me a Coffee](https://www.buymeacoffee.com/suncompute)

---

**Mermaid diagrams:**

To automatically generate SVGs from Mermaid code blocks, install mermaid-cli:

```sh
npm install -g @mermaid-js/mermaid-cli
```

SVGS will be generated and embedded in your documentation if mermaid-cli (mmdc) is available.

---

**Disable SVG generation:**

You can disable automatic SVG generation from Mermaid blocks with:

```sh
./target/release/rustdocmd --generate-mermaid-svg=false
```

By default, SVGs are generated if mermaid-cli is installed.