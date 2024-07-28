use core::str;
use std::{fs, path::PathBuf};

use clap::Parser;
use itertools::Itertools;
use tree_sitter_dynamic::{DynTS, STANDARD_CAPTURE_NAMES};
use tree_sitter_highlight::{Highlight, HighlightEvent};

#[derive(Debug, Parser)]
struct Args {
    lang: String,
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    let mut ts = DynTS::new(&args.lang, STANDARD_CAPTURE_NAMES)?;

    let contents = fs::read(&args.file)?;

    print!("<pre>");
    for event in ts.highlight(&contents) {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let content = str::from_utf8(&contents[start..end])?;
                let encoded = html_escape::encode_text(content);
                print!("{}", encoded);
            }
            HighlightEvent::HighlightStart(Highlight(n)) => {
                let name = STANDARD_CAPTURE_NAMES[n].split('.').join(" ");
                print!("<span class=\"{}\">", name);
            }
            HighlightEvent::HighlightEnd => {
                print!("</span>")
            }
        }
    }
    println!("</pre>");

    Ok(())
}
