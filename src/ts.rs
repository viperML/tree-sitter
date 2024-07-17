use std::{env, ffi::CString, fs::File, os::fd::AsRawFd, path::PathBuf};

use once_cell::sync::Lazy;
use tree_sitter::Language;

use crate::dl::Symbol;

pub fn load_language<S>(name: S) -> Language
where
    S: AsRef<str>,
{
    let name = name.as_ref();
    let grammar_path = PathBuf::from(env::var("TS_GRAMMAR_PATH").unwrap());

    let f = File::options()
        .read(true)
        .open(
            grammar_path
                .join(format!("tree-sitter-{name}-grammar"))
                .join("parser"),
        )
        .unwrap();

    let lib =
        unsafe { crate::dl::Library::open(format!("/proc/self/fd/{}", f.as_raw_fd())) }.unwrap();

    let sym_name = CString::new(format!("tree_sitter_{name}")).unwrap();
    let sym: Symbol<extern "C" fn() -> Language> = unsafe { lib.get(sym_name) }.unwrap();

    let res = unsafe { sym.as_raw() }();
    return res;
}
