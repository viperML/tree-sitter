mod modeline;
mod path;

use std::{env, fs};

use libloading::{Library, Symbol};
use path::find_in_path;
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use self::modeline::Modeline;

const BASE: &str = "TS_GRAMMAR_PATH";

pub struct DynTS {
    _lib: Library,
    language: Language,
    highlight_config: HighlightConfiguration,
    highlighter: Highlighter,

    recognized_names: Vec<String>,
}

type It = Result<HighlightEvent, tree_sitter_highlight::Error>;

pub struct Highlights<'a> {
    iter: Box<dyn Iterator<Item = It> + 'a>,
}

impl DynTS {
    pub fn new<S>(language: S, recognized_names: &[impl AsRef<str>]) -> eyre::Result<Self>
    where
        S: AsRef<str>,
    {
        let l_name = language.as_ref();
        let grammar_path = env::var(BASE)?;

        let lib =
            unsafe { Library::new(find_in_path(&grammar_path, format!("parser/{l_name}.so"))?)? };

        let symbol_name = format!("tree_sitter_{l_name}");
        let symbol: Symbol<unsafe extern "C" fn() -> Language> =
            unsafe { lib.get(symbol_name.as_bytes())? };

        let language = unsafe { symbol() };

        let mut highlights =
            find_in_path(&grammar_path, format!("queries/{l_name}/highlights.scm"))
                .ok()
                .and_then(|p| fs::read_to_string(p).ok())
                .unwrap_or_default();

        let injections = find_in_path(&grammar_path, format!("queries/{l_name}/injections.scm"))
            .ok()
            .and_then(|p| fs::read_to_string(p).ok())
            .unwrap_or_default();

        let locals = find_in_path(&grammar_path, format!("queries/{l_name}/locals.scm"))
            .ok()
            .and_then(|p| fs::read_to_string(p).ok())
            .unwrap_or_default();

        let highlights_modeline = Modeline::get(&highlights);

        for sublang in &highlights_modeline.inherits {
            let subhighlights =
                find_in_path(&grammar_path, format!("queries/{sublang}/highlights.scm"))
                    .ok()
                    .and_then(|p| fs::read_to_string(p).ok())
                    .unwrap_or_default();

            highlights = [subhighlights, highlights].join("\n");
        }

        let mut config = HighlightConfiguration::new(
            unsafe { symbol() },
            l_name,
            &highlights,
            &injections,
            &locals,
        )?;

        config.configure(recognized_names);

        Ok(DynTS {
            _lib: lib,
            language,
            highlight_config: config,
            highlighter: Highlighter::new(),
            recognized_names: recognized_names
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
        let s: Vec<_> = self.recognized_names.iter().map(|s| s.as_str()).collect();

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

#[test]
fn test_path() {
    let ts = DynTS::new("javascript", &["attribute"]).unwrap();
    _ = ts.language.version();
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
