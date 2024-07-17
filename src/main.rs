#![deny(unsafe_op_in_unsafe_fn)]
mod dl;

use std::os::fd::AsRawFd;
use std::{env, ffi::CString, fs::File, path::PathBuf};

use dl::{Library, Symbol};
use tree_sitter::Language;

fn main() {
    let lang = "javascript";
    let grammar_path = env::var("TS_GRAMMAR_PATH").unwrap_or(String::from("result"));

    let f = File::options()
        .read(true)
        .open(
            PathBuf::from(grammar_path)
                .join(format!("tree-sitter-{lang}-grammar"))
                .join("parser"),
        )
        .unwrap();

    let path = format!("/proc/self/fd/{}", f.as_raw_fd());
    println!("{path:?}");

    let lib = unsafe { Library::open(path) }.unwrap();

    println!("{lib:?}");

    let sym_name = CString::new(format!("tree_sitter_{lang}")).unwrap();
    let sym: Symbol<extern "C" fn() -> Language> = unsafe { lib.get(sym_name) }.unwrap();
    println!("{sym:?}");

    let l = unsafe { sym.as_raw() }();
    println!("{l:?}");
}
