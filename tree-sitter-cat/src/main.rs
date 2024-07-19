use std::io::{stdout, Write};

use tree_sitter_dynamic::DynTS;
use tree_sitter_highlight::{HighlightEvent, Highlighter};

fn main() -> eyre::Result<()> {
    let js = unsafe {
        DynTS::new(
            "javascript",
            &[
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
            ],
        )?
    };

    let mut highlighter = Highlighter::new();

    let s =  b"const foo = bar;";

    let highlights =
        highlighter.highlight(js.highlight_config(), b"const foo = bar;", None, |_| None)?;


    let mut color = false;
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                // eprintln!("source: {}-{}", start, end);
                for i in start..end {
                    stdout().write_all(&[ s[i] ])?;
                }

            }
            HighlightEvent::HighlightStart(s) => {
                // eprintln!("highlight style started: {:?}", s);
                if color {
                    print!("\x1b[31m");
                    color = false;
                } else {
                    print!("\x1b[0m");
                    color = true;
                }
            }
            HighlightEvent::HighlightEnd => {
                // eprintln!("highlight style ended");
            }
        }
    }

    Ok(())
}
