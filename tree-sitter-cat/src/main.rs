use std::io::{stdout, Write};

use tree_sitter_dynamic::DynTS;
use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    let names = [
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];

    let js = unsafe { DynTS::new("javascript", &names)? };

    let mut highlighter = Highlighter::new();

    let s = br#"import { getCollection } from "astro:content";
import Post from "./Post.astro";

const blogEntries = (await getCollection("blog"))
    .sort((a, b) => {
        return b.data.pubDate.getTime() - a.data.pubDate.getTime();
    })
    .filter((elem) => {
        if (elem.data.draft === true) {
            return false;
        } else {
            return true;
        }
    });"#;

    let highlights = highlighter.highlight(js.highlight_config(), s, None, |_| None)?;

    let mut color = false;
    print!("<pre>");
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                for i in start..end {
                    stdout().write_all(&[s[i]])?;
                }
            }
            HighlightEvent::HighlightStart(Highlight(n)) => {
                let name = names[n].split(".").join(" ");
                print!("<span class=\"{}\">", name);
            }
            HighlightEvent::HighlightEnd => {
                print!("</span>")
            }
        }
    }
    print!("</pre>");

    Ok(())
}
