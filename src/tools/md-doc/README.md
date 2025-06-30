# md-doc

`md-doc` converts rustdoc JSON output into Markdown. It is a prototype tool for the Markdown Doc V2 effort.

## Generating JSON

Run rustdoc in JSON mode with nightly Rust:

```sh
RUSTDOCFLAGS="-Z unstable-options --output-format json" \
    cargo +nightly doc --no-deps
```

This creates `target/doc/<crate>.json` for your crate.

## Running md-doc

Invoke the tool with the JSON file as input:

```sh
cargo run -p md-doc -- target/doc/<crate>.json -o target/md/<crate>.md
```

The output Markdown file is written to `target/md/` by default when `-o` is not specified.
