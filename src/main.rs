#![deny(unsafe_op_in_unsafe_fn)]
mod dl;
// mod ts;

use std::mem;
use std::os::fd::AsRawFd;
use std::process::exit;
use std::{env, ffi::CString, fs::File, path::PathBuf};

use dl::{Library, Symbol};
use libc::{dlopen, dlsym, CS, RTLD_NOW};
use tree_sitter::Language;
use tree_sitter_highlight::{HighlightEvent, Highlighter};
// use ts::{load_highlighter_config, load_language};

fn main() {
    let lang = "javascript";

    // let l = load_language("html");
    // println!("{l:?}");
    // let v = l.version();

    //     let config = load_highlighter_config(lang);

    //     let mut highlighter = Highlighter::new();

    //     let highlights = highlighter
    //         .highlight(&config, b"const x = new Y();", None, |_| None)
    //         .unwrap();

    //     for event in highlights {
    //         match event.unwrap() {
    //             HighlightEvent::Source { start, end } => {
    //                 eprintln!("source: {}-{}", start, end);
    //             }
    //             HighlightEvent::HighlightStart(s) => {
    //                 eprintln!("highlight style started: {:?}", s);
    //             }
    //             HighlightEvent::HighlightEnd => {
    //                 eprintln!("highlight style ended");
    //             }
    //         }
    //     }

    // let f = File::open("result/tree-sitter-javascript-grammar/parser").unwrap();
    // let s = CString::new(format!("/proc/self/fd/{}", f.as_raw_fd())).unwrap();

    // let lib_ptr = unsafe { dlopen(s.as_ptr() as _, RTLD_NOW) };
    // if lib_ptr.is_null() {
    //     panic!()
    // }

    // drop(f);
    let lib = {
        let f = File::open("result/tree-sitter-javascript-grammar/parser").unwrap();
        let name = format!("/proc/self/fd/{}", f.as_raw_fd());
        unsafe { Library::open(name).unwrap() }
    };

    let sym: Symbol<extern "C" fn() -> Language> =
        unsafe { lib.get(CString::new("tree_sitter_javascript").unwrap()) }.unwrap();

    println!("{sym:?}");
    unsafe {
        sym.flat_map(|sym| {
            let l = sym();
            println!("{l:?}");
            // let v = l.version();
            // println!("{v:?}");
        })
    };

    // let sym_name = CString::new(format!("tree_sitter_javascript")).unwrap();
    // let sym = unsafe { dlsym(lib.ptr, sym_name.as_ptr() as _) };
    // if sym.is_null() {
    //     panic!()
    // };

    // let f: extern "C" fn() -> Language = unsafe { mem::transmute(sym) };
    // let l = f();

    // println!("{l:?}");

    // let v = l.version();
    // println!("{v:?}");

    // let sym_name = CString::new("tree_sitter_javascript").unwrap();
    // let sym2: Symbol<extern "C" fn() -> Language> = unsafe { lib.get(sym_name) }.unwrap();

    // let x = unsafe { sym2.as_raw() }();
    // println!("{x:?}");
    // let v = x.version();
    // println!("{v:?}");

    // let l = load_language("javascript");
    // let v = l.version();
    // println!("{v:?} @{}:{}", file!(), line!());

    // dlopen(, flag)
}
