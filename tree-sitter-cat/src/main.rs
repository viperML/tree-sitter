use core::str;
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

use clap::Parser;
use serde::Deserialize;
use tree_sitter_dynamic::{DynTS, STANDARD_CAPTURE_NAMES};
use tree_sitter_highlight::{Highlight, HighlightEvent};

#[derive(Debug, Parser)]
/// treesitter-powered cat clone
struct Args {
    /// Which language the source code is written in
    language: String,
    /// File to colorize
    file: PathBuf,

    #[arg(long, short)]
    /// Output information about the tree-sitter highlights
    debug: bool,

    #[arg(short, long)]
    /// Path to an alternative stylesheet
    stylesheet: Option<PathBuf>,
}

const DEFAULT_STYLESHEET: &str = include_str!("terminal.toml");

#[derive(Debug, Deserialize)]
struct StyleSheet {
    prefix: Option<String>,
    end: Option<String>,
    theme: HashMap<String, String>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let mut ts = DynTS::new(&args.language, STANDARD_CAPTURE_NAMES)?;

    let contents = fs::read(&args.file)?;

    let stylesheet: StyleSheet = match args.stylesheet {
        Some(p) => toml::from_str(&fs::read_to_string(p)?),
        None => toml::from_str(DEFAULT_STYLESHEET),
    }?;

    let prefix = &stylesheet.prefix.unwrap_or_default();
    let end = &stylesheet.end.unwrap_or_default();

    for event in ts.highlight(&contents) {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                let content = str::from_utf8(&contents[start..end])?;
                // let encoded = html_escape::encode_text(content);
                print!("{}", content);
            }
            HighlightEvent::HighlightStart(Highlight(n)) => {
                let capture = STANDARD_CAPTURE_NAMES[n];

                if let Some(s) = stylesheet.theme.get(capture) {
                    print!("{prefix}{s}");
                }

                if args.debug {
                    print!("({capture} ");
                }
            }
            HighlightEvent::HighlightEnd => {
                if args.debug {
                    print!(")");
                }
                print!("{end}");
            }
        }
    }

    Ok(())
}
