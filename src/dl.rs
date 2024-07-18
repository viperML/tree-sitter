use std::{
    ffi::{CStr, CString, OsStr},
    fmt::{self},
    marker::PhantomData,
};

use libc::{c_void, dlclose, dlerror, dlopen, dlsym, RTLD_NOW};

#[derive(Debug)]
pub struct Library {
    pub ptr: *mut c_void,
}

#[derive(Debug)]
pub struct LibError(String);

impl fmt::Display for LibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LibError {}

#[derive(Debug)]
pub struct Symbol<'lib, T> {
    ptr: *mut c_void,
    _phantom: PhantomData<&'lib T>,
}

impl Library {
    pub unsafe fn open<S: AsRef<OsStr>>(filename: S) -> Result<Self, LibError> {
        let filename = filename.as_ref();
        let s = filename.as_encoded_bytes();

        let ptr = unsafe { dlopen(s.as_ptr() as _, RTLD_NOW) };

        if ptr.is_null() {
            let error = unsafe { CString::from_raw(dlerror()) };
            Err(LibError(error.to_str().unwrap().to_owned()))
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn get<T, S>(&self, symbol: S) -> Result<Symbol<'_, T>, LibError>
    where
        S: AsRef<CStr>,
    {
        let s = symbol.as_ref();

        let ptr = unsafe { dlsym(self.ptr, s.as_ptr() as _) };

        if ptr.is_null() {
            let error = unsafe { CString::from_raw(dlerror()) };
            Err(LibError(error.to_str().unwrap().to_owned()))
        } else {
            Ok(Symbol {
                ptr,
                _phantom: PhantomData,
            })
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { dlclose(self.ptr) };
    }
}

impl<'lib, T> Symbol<'lib, T> {
    pub unsafe fn flat_map<F, R>(&self, func: F)
    where
        F: FnOnce(&T) -> R,
    {
        let casted = unsafe { &*(&self.ptr as *const *mut _ as *const T) };
        func(casted);
    }
}

// impl<'lib, T> Symbol<'lib, T> {
//     pub unsafe fn as_raw(&self) -> &T {
//         unsafe { &*(&self.ptr as *const *mut _ as *const T) }
//     }
// }
