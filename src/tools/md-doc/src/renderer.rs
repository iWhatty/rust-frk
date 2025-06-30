use rustdoc_json_types::{Crate, Item, ItemEnum};

pub trait DocRenderer {
    fn render_crate(&self, krate: &Crate) -> String;
    fn render_item(&self, item: &Item) -> String;
}

#[derive(Default)]
pub struct MarkdownRenderer;

impl DocRenderer for MarkdownRenderer {
    fn render_crate(&self, krate: &Crate) -> String {
        let mut out = String::new();
        if let Some(root) = krate.index.get(&krate.root) {
            if let Some(name) = &root.name {
                out.push_str(&format!("# {}\n\n", name));
            } else {
                out.push_str("# Crate\n\n");
            }
            if let Some(docs) = &root.docs {
                out.push_str(docs);
                out.push_str("\n\n");
            }
            if let ItemEnum::Module(m) = &root.inner {
                for id in &m.items {
                    if let Some(item) = krate.index.get(id) {
                        out.push_str(&self.render_item(item));
                    }
                }
            }
        }
        out
    }

    fn render_item(&self, item: &Item) -> String {
        let mut out = String::new();
        if let Some(name) = &item.name {
            out.push_str(&format!("## `{}`\n\n", name));
        }
        if let Some(docs) = &item.docs {
            out.push_str(docs);
            out.push_str("\n\n");
        }
        out
    }
}
