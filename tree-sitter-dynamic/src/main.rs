use std::mem::{self, transmute};
use std::ptr::{self, addr_of};
use std::{ffi::CString, fs::File, os::fd::AsRawFd};

use libc::{c_void, dlclose, dlopen, dlsym, RTLD_LAZY, RTLD_LOCAL, RTLD_NOW};
use libloading::Library;
use tree_sitter::Language;

fn main() {
    let file = File::open("result/tree-sitter-javascript-grammar/parser").unwrap();
    let name = CString::new(format!("/proc/self/fd/{}", file.as_raw_fd())).unwrap();

    #[cfg(no)]
    unsafe {
        let lib = libloading::Library::new("result/tree-sitter-javascript-grammar/parser").unwrap();
        let lib2 = libloading::Library::new("result/tree-sitter-javascript-grammar/parser").unwrap();

        let func: libloading::Symbol<unsafe extern "C" fn() -> Language> =
            lib.get(b"tree_sitter_javascript").unwrap();

        let lang = func();
        let v = lang.version();
        println!("{lang:?}->{v}");

        lib2.close().unwrap();

        let lang = func();
        let v = lang.version();
        println!("{lang:?}->{v}");
    }

    #[cfg(no)]
    unsafe {
        let f = File::open("result/tree-sitter-javascript-grammar/parser").unwrap();
        let path = CString::new(format!("/proc/self/fd/{}", f.as_raw_fd())).unwrap();
        let lib_ptr = dlopen(path.as_ptr(), RTLD_NOW);
        if lib_ptr.is_null() {
            panic!();
        }
        println!("{lib_ptr:x?}");

        let f = File::open("result/tree-sitter-javascript-grammar/parser").unwrap();
        let path = CString::new(format!("/proc/self/fd/{}", f.as_raw_fd())).unwrap();
        let lib_ptr2 = dlopen(path.as_ptr(), RTLD_NOW);
        if lib_ptr2.is_null() {
            panic!();
        }
        println!("{lib_ptr2:x?}");

        let func_ptr = dlsym(lib_ptr, c"tree_sitter_javascript".as_ptr());
        if func_ptr.is_null() {
            panic!();
        }
        let func: unsafe extern "C" fn() -> Language = transmute(func_ptr);

        let lang = func();
        let v = lang.version();
        println!("{lang:?}->{v}");

        // lib2.close().unwrap();
        dlclose(lib_ptr);

        let lang = func();
        let v = lang.version();
        println!("{lang:?}->{v}");
    }

    unsafe {
        let lib_ptr = dlopen(name.as_ptr() as _, RTLD_LAZY | RTLD_LOCAL);
        println!("1: {lib_ptr:x?}");

        let sym = dlsym(lib_ptr, c"tree_sitter_javascript".as_ptr());

        let lib: unsafe extern "C" fn() -> Language = mem::transmute(sym);
        let lib = lib();
        let _v = lib.version();


        let lib_ptr2 = dlopen(name.as_ptr() as _, RTLD_LAZY | RTLD_LOCAL);
        println!("1: {lib_ptr:x?}");

        dlclose(lib_ptr2);
        dlclose(lib_ptr2);
        println!("closed");

        let _v = lib.version();
    }
}
