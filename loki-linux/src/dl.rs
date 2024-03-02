use std::ffi::{c_char, c_int, c_void, CStr};
use std::rc::Rc;

// https://github.com/nagisa/rust_libloading/blob/master/src/os/unix/consts.rs
pub const RTLD_LAZY: c_int = 1;
pub const RTLD_NOW: c_int = 2;

// Link to the dynamic linker & its functions
// Docs: https://linux.die.net/man/3/dlopen
#[link(name = "dl")]
extern "C" {
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlerror() -> *const c_char;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
}

/// A safe wrapper around dlerror
pub fn get_dlerror() -> Option<Rc<str>> {
    let err = unsafe { dlerror() };
    if err.is_null() {
        None
    } else {
        let c_str = unsafe { CStr::from_ptr(err) };
        Some(c_str.to_str().unwrap().into())
    }
}
