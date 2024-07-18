use std::{
    env,
    ffi::CString,
    fs::{self, File},
    os::fd::AsRawFd,
    path::PathBuf,
};

use once_cell::sync::Lazy;
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

use crate::dl::Symbol;

pub fn load_language<S>(name: S) -> Language
where
    S: AsRef<str>,
{
    let name = name.as_ref();
    let grammar_path = PathBuf::from(env::var("TS_GRAMMAR_PATH").unwrap());

    let file = File::options()
        .read(true)
        .open(
            grammar_path
                .join(format!("tree-sitter-{name}-grammar"))
                .join("parser"),
        )
        .unwrap();

    let library =
        unsafe { crate::dl::Library::open(format!("/proc/self/fd/{}", file.as_raw_fd())) }.unwrap();

    let sym_name = CString::new(format!("tree_sitter_{name}")).unwrap();
    let sym: Symbol<extern "C" fn() -> Language> = unsafe { library.get(sym_name) }.unwrap();

    let res = unsafe { sym.as_raw() }();

    println!("{:?} @{}:{}", res.version(), file!(), line!());

    return res;
}

pub fn load_highlighter_config<S>(name: S) -> HighlightConfiguration
where
    S: AsRef<str>,
{
    let name = name.as_ref();

    let language = load_language(name);
    let v = language.version();
    println!("{language:?}, {v}");

    let grammar_path = PathBuf::from(env::var("TS_GRAMMAR_PATH").unwrap())
        .join(format!("tree-sitter-{name}-grammar"));

    let highlights_query =
        fs::read_to_string(grammar_path.join("queries").join("highlights.scm")).unwrap();

    let injections_query =
        fs::read_to_string(grammar_path.join("queries").join("injections.scm")).unwrap();

    let locals_query = fs::read_to_string(grammar_path.join("queries").join("locals.scm")).unwrap();

    let mut config = HighlightConfiguration::new(
        language,
        name,
        &highlights_query,
        &injections_query,
        &locals_query,
    )
    .unwrap();

    config.configure(&[
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
    ]);

    return config;
}
