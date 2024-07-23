use core::str;
use std::io::{stdout, Write};

use itertools::Itertools;
use tree_sitter_dynamic::DynTS;
use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};

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

    let js = unsafe { DynTS::new("typescript", &names)? };

    let mut highlighter = Highlighter::new();

    let s = br#"
export function formatDate(date: Date): string {
    return date.toLocaleDateString("en-uk", {
        year: "numeric",
        month: "long",
        day: "numeric",
    })
}
"#;

    let highlights = highlighter.highlight(js.highlight_config(), s, None, |_| None)?;

    print!("<pre>");
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let content = str::from_utf8(&s[start..end])?;
                let encoded = html_escape::encode_text(content);
                print!("{}", encoded);
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
