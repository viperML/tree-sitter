mod detect;
mod modeline;
mod path;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs};

use eyre::eyre;
use libloading::{Library, Symbol};
use serde::Deserialize;
use serde_with::serde_as;
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use serde_with::formats::PreferMany;
use serde_with::OneOrMany;

pub use detect::detect_language;

const BASE: &str = "TS_GRAMMAR_PATH";

pub struct DynTS {
    _lib: Library,
    language: Language,
    highlight_config: HighlightConfiguration,
    highlighter: Highlighter,

    capture_names: Vec<String>,
}

type It = Result<HighlightEvent, tree_sitter_highlight::Error>;

pub struct Highlights<'a> {
    iter: Box<dyn Iterator<Item = It> + 'a>,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    #[serde(rename = "tree-sitter")]
    tree_sitter: Vec<TreeSitterConfig>,
}

// TODO: implement default path

#[serde_as]
#[derive(Debug, Deserialize)]
struct TreeSitterConfig {
    path: Option<PathBuf>,
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    highlights: Vec<PathBuf>,
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    injections: Vec<PathBuf>,
    #[serde(default)]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    locals: Vec<PathBuf>,
    #[serde(default, rename = "file-types")]
    #[serde_as(as = "OneOrMany<_, PreferMany>")]
    file_types: Vec<String>,
}

#[derive(Debug)]
struct Grammar {
    parser: PathBuf,
    highlights: String,
    locals: String,
    injections: String,
}

/// The weird grammar lookup from tree-sitter upstream
fn find_grammar<S, P>(language: S, ts_grammar_path: P) -> eyre::Result<Grammar>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let language = language.as_ref();

    for grammar in fs::read_dir(ts_grammar_path)? {
        let grammar = grammar?.path();

        let package_json = grammar.join("package.json");

        if let Ok(file) = File::open(package_json) {
            let parsed: PackageJson = serde_json::from_reader(file)?;

            for config in parsed.tree_sitter {
                let this_language: String = match &config.path {
                    Some(p) => p.file_name().unwrap().to_os_string().into_string().unwrap(),
                    None => grammar
                        .file_name()
                        .unwrap()
                        .to_owned()
                        .into_string()
                        .unwrap()
                        .strip_prefix("tree-sitter-")
                        .unwrap()
                        .to_owned(),
                };

                let parser: PathBuf = match &config.path {
                    Some(p) => grammar.join(p).join("parser.so"),
                    None => grammar.join(format!("{this_language}.so")),
                };

                if !(language == this_language && parser.exists()) {
                    continue;
                }

                let mut highlights = String::new();
                for relpath in config.highlights {
                    let path = grammar.join(relpath);
                    highlights.push_str(&fs::read_to_string(path)?);
                }

                let mut injections = String::new();
                for relpath in config.injections {
                    injections.push_str(&fs::read_to_string(grammar.join(relpath))?);
                }

                let mut locals = String::new();
                for relpath in config.locals {
                    locals.push_str(&fs::read_to_string(grammar.join(relpath))?);
                }

                return Ok(Grammar {
                    parser,
                    highlights,
                    injections,
                    locals,
                });
            }
        }
    }

    Err(eyre!("Couldn't find language {language}"))
}

impl DynTS {
    pub fn new<S>(language: S, capture_names: &[impl AsRef<str>]) -> eyre::Result<Self>
    where
        S: AsRef<str>,
    {
        let language = language.as_ref();
        let ts_grammar_path = env::var(BASE)?;

        let grammar = find_grammar(language, ts_grammar_path)?;

        let lib = unsafe { Library::new(&grammar.parser)? };

        let symbol_name = format!("tree_sitter_{language}");
        let symbol: Symbol<unsafe extern "C" fn() -> Language> =
            unsafe { lib.get(symbol_name.as_bytes())? };

        let ts_language = unsafe { symbol() };

        let mut config = HighlightConfiguration::new(
            unsafe { symbol() },
            language,
            &grammar.highlights,
            &grammar.injections,
            &grammar.locals,
        )?;

        config.configure(capture_names);

        Ok(DynTS {
            _lib: lib,
            language: ts_language,
            highlight_config: config,
            highlighter: Highlighter::new(),
            capture_names: capture_names
                .iter()
                .map(|s| String::from(s.as_ref()))
                .collect(),
        })
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub fn highlight_config(&self) -> &HighlightConfiguration {
        &self.highlight_config
    }

    pub fn highlight<'se, 'so>(&'se mut self, source: &'so [u8]) -> Highlights
    where
        'so: 'se,
    {
        // FIXME: hack to pass down the capture names
        let s: Vec<_> = self.capture_names.iter().map(|s| s.as_str()).collect();

        let iter = self
            .highlighter
            .highlight(&self.highlight_config, source, None, move |injected| {
                let new = DynTS::new(injected, s.as_slice()).ok()?;
                let snew: &'static _ = Box::leak(Box::new(new));

                Some(&snew.highlight_config)
            })
            .unwrap();

        let iter = Box::new(iter);

        Highlights { iter }
    }
}

impl<'a> Iterator for Highlights<'a> {
    type Item = It;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub const STANDARD_CAPTURE_NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "carriage-return",
    "comment",
    "comment.documentation",
    "constant",
    "constant.builtin",
    "constructor",
    "constructor.builtin",
    "embedded",
    "error",
    "escape",
    "function",
    "function.builtin",
    "keyword",
    "markup",
    "markup.bold",
    "markup.heading",
    "markup.italic",
    "markup.link",
    "markup.link.url",
    "markup.list",
    "markup.list.checked",
    "markup.list.numbered",
    "markup.list.unchecked",
    "markup.list.unnumbered",
    "markup.quote",
    "markup.raw",
    "markup.raw.block",
    "markup.raw.inline",
    "markup.strikethrough",
    "module",
    "number",
    "operator",
    "property",
    "property.builtin",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regexp",
    "string.special",
    "string.special.symbol",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
];
