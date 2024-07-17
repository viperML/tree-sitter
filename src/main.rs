use tree_sitter::{Language, Parser};

use libc::{dlopen, dlsym, RTLD_NOW};

fn main() {
    let x = unsafe { dlopen(c"parser".as_ptr(), RTLD_NOW) };
    println!("{:x?}", x);

    if x.is_null() {
        println!("Couldn't dlopen");
        return;
    }

    let y = unsafe { dlsym(x, c"tree_sitter_html".as_ptr()) };
    println!("{:x?}", y);

    let z: extern "C" fn() -> Language = unsafe { core::mem::transmute(y) };

    let lang = z();
    println!("{:?}", lang);
    println!("{:?}", lang.version());
}
