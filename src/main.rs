use std::{env, error::Error, fs::File, path::PathBuf};

use tree_sitter::{Language, Parser};

use libc::{dlopen, dlsym, RTLD_NOW};
use std::os::fd::AsRawFd;

fn main() {
    let grammar_path = env::var("TS_GRAMMAR_PATH").unwrap_or(String::from("result"));

    let f = File::options()
        .read(true)
        .open(PathBuf::from(grammar_path).join("tree-sitter-html-grammar/parser"))
        .unwrap();

    let path = format!("/proc/self/fd/{}", f.as_raw_fd());
    println!("{path:?}");

    let x = unsafe { dlopen(path.as_ptr() as _, RTLD_NOW) };
    println!("{:x?}", x);

    if x.is_null() {
        panic!("Couldn't dlopen");
    }

    let y = unsafe { dlsym(x, c"tree_sitter_html".as_ptr()) };
    println!("{:x?}", y);
    if y.is_null() {
        panic!("dlsym null");
    }

    let z: extern "C" fn() -> Language = unsafe { core::mem::transmute(y) };

    let lang = z();
    println!("{:?}", lang);
    println!("{:?}", lang.version());
}
