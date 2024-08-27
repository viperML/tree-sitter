use core::str;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, stdout, BufReader, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use is_terminal::IsTerminal;
use magic::cookie::Flags as CookieFlags;
use serde::Deserialize;
use tree_sitter_dynamic::{DynTS, STANDARD_CAPTURE_NAMES};
use tree_sitter_highlight::{Highlight, HighlightEvent};

#[derive(Debug, Parser)]
/// Tree-sitter-powered cat clone
struct Args {
    /// File to colorize
    file: PathBuf,

    /// Language to use
    #[arg(short, long)]
    language: Option<String>,

    /// Output information about the tree-sitter highlights
    #[arg(long, short)]
    debug: bool,

    /// Path to an alternative stylesheet
    #[arg(short, long)]
    stylesheet: Option<PathBuf>,
}

const DEFAULT_STYLESHEET: &str = include_str!("terminal.toml");

#[derive(Debug, Deserialize)]
struct StyleSheet {
    prefix: Option<String>,
    end: Option<String>,
    theme: HashMap<String, String>,
}

fn regular_cat<P>(path: P) -> eyre::Result<()>
where
    P: AsRef<Path>,
{
    let p = path.as_ref();

    let mut reader = BufReader::new(File::open(p)?);
    io::copy(&mut reader, &mut stdout())?;
    return Ok(());
}

fn main() -> eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()?;

    let args = Args::parse();

    if !stdout().is_terminal() {
        regular_cat(&args.file)?;
        return Ok(());
    }

    let cookie = magic::Cookie::open(CookieFlags::MIME_ENCODING | CookieFlags::SYMLINK)?;
    let database = Default::default();
    let cookie = cookie.load(&database).unwrap();
    let file_result = cookie.file(&args.file)?;

    if file_result == "binary" {
        return Err(eyre::eyre!("Reading binary file, refusing to operate"));
    }

    let language = match args.language {
        Some(l) => l,
        None => match tree_sitter_dynamic::detect_language(
            std::env::var("TS_GRAMMAR_PATH")?,
            args.file.as_path(),
        ) {
            Ok(ok) => ok,
            Err(_) => {
                regular_cat(&args.file)?;
                return Ok(());
            }
        },
    };

    let mut ts = DynTS::new(language, STANDARD_CAPTURE_NAMES)?;

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
