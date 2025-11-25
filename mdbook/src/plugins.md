## Mermaid diagrams & SVG generation

To automatically generate SVGs from Mermaid code blocks, install mermaid-cli:

```sh
npm install -g @mermaid-js/mermaid-cli
```

SVGs will be generated and embedded in your documentation if mermaid-cli (mmdc) is available.

---

**Disable SVG generation:**

You can disable automatic SVG generation from Mermaid blocks with:

```sh
./target/release/rustdocmd --generate-mermaid-svg=false
```

By default, SVGs are generated if mermaid-cli is installed.