use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use fs_err as fs;
use rustdoc_json_types::Crate;

mod renderer;
use renderer::{DocRenderer, MarkdownRenderer};

#[derive(Parser)]
struct Options {
    /// Input JSON produced by `rustdoc --output-format json`
    input: PathBuf,
    /// Output Markdown file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Options::parse();
    let data = fs::read_to_string(&opts.input)?;
    let krate: Crate = serde_json::from_str(&data)?;
    let renderer = MarkdownRenderer::default();
    let md = renderer.render_crate(&krate);
    let out_path = opts.output.unwrap_or_else(|| {
        let mut p = PathBuf::from("target/md");
        if let Some(stem) = opts.input.file_stem() {
            p.push(stem);
        } else {
            p.push("output");
        }
        p.set_extension("md");
        p
    });
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_path, md)?;
    Ok(())
}
