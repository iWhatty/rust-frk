# MARKDOWN\_DOC\_V2 — Future Plan for rustdoc Markdown Export

## 🌐 Goal

Build **V2** of the rustdoc-to-Markdown system:

* Abstract over multiple formats (`md`, `typst`, `latex`, etc.)
* Introduce configurable output templates
* Retain simplicity, no runtime JS needed
* Hook cleanly into the `rustdoc` pipeline or serve as a composable side-tool

---

## 🧠 Mission Objective

Enable `rustdoc` to export documentation in **Markdown** format—either as an official feature or an external companion tool—by leveraging its existing JSON output pipeline. The goal is clean, maintainable, and upstream-viable Markdown generation for Rust crate documentation.

---

## 🎯 Primary Goals

* **G1: Generate single-file Markdown documentation** per crate.
* **G2: Reuse existing rustdoc infrastructure (preferably the JSON output).**
* **G3: Make the approach maintainable, modular, and merge-friendly.**
* **G4: Provide CLI usability either via ****`cargo doc --md`**** or a standalone tool.**
* **G5: Ensure links, code blocks, headers, and intra-doc references render well.**

---

## 🚧 Architecture Overview

### Core Components

* **Parser Engine**

  * Input: rustdoc JSON output
  * Parses crate/module/struct/function/type docs
* **Renderer Interface**

  * Trait: `DocRenderer { fn render(&self, item: &DocItem) -> String }`
  * Implementations:

    * `MarkdownRenderer`
    * `TypstRenderer`
    * `LaTeXRenderer` (optional)
* **Template Layer**

  * Allow custom themes (e.g., header levels, TOC behavior, code block styling)
  * Simple `*.toml` or `*.json` config

### CLI Tool

```
cargo md-doc --format md       # single .md file
cargo md-doc --format typst    # .typ file output
```

---

## 🧪 Output Design Goals

* ✅ Human-readable
* ✅ Supports Git diffs
* ✅ Keeps API structure: modules, impls, enums, traits
* ✅ Code blocks are fenced and language-tagged
* ✅ Doc links translated into local anchors or plaintext fallback
* 🟨 TOC support (opt-in)
* 🟨 Per-item collapse/toggle metadata (for downstream renderers)

---

##

---

## 🔒 Constraints

* Don’t break existing `cargo doc` behavior.
* Markdown should not require JavaScript to render.
* Output must be deterministic, git-friendly, and readable.

---

## 🧬 Key Subsystems in rustdoc to Study

* [`src/librustdoc/clean/`](https://github.com/rust-lang/rust/tree/master/src/librustdoc/clean): Where Rust AST is cleaned into internal doc models.
* [`src/librustdoc/json/`](https://github.com/rust-lang/rust/tree/master/src/librustdoc/json): The existing JSON output logic—target for reuse or parallel output.
* [`src/librustdoc/html/`](https://github.com/rust-lang/rust/tree/master/src/librustdoc/html): Existing HTML backend—*do not modify*, but study structure.
* `DocContext`, `Item`, `Crate`, and `Type` types – central in defining what data gets rendered.

---

## 🛠️ Current Plan of Attack

### Phase 1: Proof-of-Concept (PoC)

## 🧠 Notes for CODEX Agents

* **Avoid HTML parsing.** JSON is your primary doc source.
* **No need to reimplement rustc parsing or typechecking.** Use `cargo doc` to output JSON.
* Use **Rust nightly** for JSON support:

  ```sh
  RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo +nightly doc --no-deps
  ```
* For Markdown:

  * Use `#` for crate/module headers.
  * Use ` ```rust ` for code examples.
  * Use `[]()` links where available.
  * Strip non-doc metadata unless explicitly required.

- Use `rustdoc --output-format json` (requires `+nightly`).
- Parse the output and generate Markdown via an external tool (e.g., Rust binary or Python script).
- Output format: `target/md/your_crate.md`

### Phase 2: Codex Integration

* Codex agents should:

  * Navigate the [`rust-lang/rust`](https://github.com/rust-lang/rust) repo.
  * Focus on the `src/librustdoc` submodules.
  * Understand `rustdoc_json_types.rs` for output structure.
  * Identify which internal structs or traits could support a new Markdown “renderer.”
  * Propose minimal patches that hook into existing pipelines.

### Phase 3: RFC + Upstream Strategy

* Prepare upstream pitch (if Phase 2 output is clean):

  * Justify Markdown as a general-purpose format.
  * Show real-world use cases: GitHub Pages, static site gen, project READMEs, CI previews.
  * Emphasize reuse of `--output-format json` to minimize diff surface.
* Optional: propose behind-a-flag mode like `--markdown` for first iteration.

---

## 🧪 PoC Task List

| Task                                                     | Agent Role     | Status         |
| -------------------------------------------------------- | -------------- | -------------- |
| Parse `target/doc/*.json` to extract crate-level doc     | ExtractorAgent | ✅ Ready        |
| Generate Markdown headers for modules / functions        | FormatterAgent | 🔄 In Progress |
| Format code blocks, docstrings, links as Markdown        | RendererAgent  | 🔄 In Progress |
| CLI wrapper: `cargo md-doc`                              | UXAgent        | ⏳ Planned      |
| Propose integration point inside `rustdoc::json::render` | ForkAgent      | ⏳ Planned      |

---
