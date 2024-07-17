#![deny(unsafe_op_in_unsafe_fn)]
mod dl;
mod ts;

use std::os::fd::AsRawFd;
use std::{env, ffi::CString, fs::File, path::PathBuf};

use dl::{Library, Symbol};
use tree_sitter::Language;
use ts::load_language;

fn main() {
    let lang = "javascript";

    let l = load_language(lang);
    println!("{l:?}");
}
