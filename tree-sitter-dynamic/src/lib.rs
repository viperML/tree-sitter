use std::{env, fs, path::PathBuf};

use libc::{dlopen, RTLD_LAZY, RTLD_LOCAL};
use libloading::{Library, Symbol};
use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;

const BASE: &'static str = "TS_GRAMMAR_PATH";

struct DynTS {
    lib: Library,
    language: Language,
    highlight_config: HighlightConfiguration,
}

impl DynTS {
    unsafe fn new<S>(language: S, recognized_names: &[impl AsRef<str>]) -> eyre::Result<Self>
    where
        S: AsRef<str>,
    {
        let l_name = language.as_ref();
        let grammar_path = env::var(BASE)?;
        let path_base =
            std::fs::canonicalize(grammar_path)?.join(format!("tree-sitter-{l_name}-grammar"));

        let lib = Library::new(path_base.join("parser"))?;

        let symbol_name = format!("tree_sitter_{l_name}");
        let symbol: Symbol<unsafe extern "C" fn() -> Language> = lib.get(symbol_name.as_bytes())?;

        let language = symbol();

        let highlights = fs::read_to_string(path_base.join("queries").join("highlights.scm"))?;
        let injections = fs::read_to_string(path_base.join("queries").join("injections.scm"))?;
        let locals = fs::read_to_string(path_base.join("queries").join("locals.scm"))?;

        let mut config =
            HighlightConfiguration::new(symbol(), l_name, &highlights, &injections, &locals)?;

        config.configure(recognized_names);

        Ok(DynTS {
            lib,
            language,
            highlight_config: config,
        })
    }

    fn language<'s>(&'s self) -> &'s Language {
        &self.language
    }

    fn highlight_config<'s>(&'s self) -> &'s HighlightConfiguration {
        &self.highlight_config
    }
}

#[test]
fn test_path() {
    let ts = unsafe { DynTS::new("javascript", &["attribute"]) }.unwrap();
    _ = ts.language.version();
}
