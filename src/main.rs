#![deny(unsafe_op_in_unsafe_fn)]
mod dl;
mod ts;

use std::os::fd::AsRawFd;
use std::{env, ffi::CString, fs::File, path::PathBuf};

use dl::{Library, Symbol};
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightEvent, Highlighter};
use ts::{load_highlighter_config, load_language};

fn main() {
    let lang = "javascript";

    let config = load_highlighter_config(lang);

    let mut highlighter = Highlighter::new();

    let highlights = highlighter
        .highlight(&config, b"const x = new Y();", None, |_| None)
        .unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                eprintln!("source: {}-{}", start, end);
            }
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", s);
            }
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            }
        }
    }
}
