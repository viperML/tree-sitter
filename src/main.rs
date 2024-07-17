use tree_sitter::{Language, Parser};

use libc::{dlopen, dlsym, RTLD_NOW};

fn main() {
    let x = unsafe { dlopen(c"tree-sitter-html/parser".as_ptr(), RTLD_NOW) };
    println!("{:x?}", x);

    if x.is_null() {
        println!("Couldn't dlopen");
        return;
    }

    let y = unsafe { dlsym(x, c"tree_sitter_html".as_ptr()) };
    println!("{:x?}", y);
    if y.is_null() {
        println!("dlsym null");
        return;
    }

    let z: extern "C" fn() -> Language = unsafe { core::mem::transmute(y) };

    let lang = z();
    println!("{:?}", lang);
    println!("{:?}", lang.version());
}
