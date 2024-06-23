//! Dynamic library loading for macOS.

use std::{
    ffi::{c_char, c_int, c_void},
    mem,
    ptr::NonNull,
};

extern "C" {
    /// Opens a library, whose symbols can then be loaded with [`dlsym`].
    pub fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    /// Loads a symbol from a library opened with [`dlopen`].
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

/// A library dynamically loaded at runtime.
pub struct Library {
    handle: NonNull<c_void>,
}
impl Library {
    /// Open a library to load symbols from.
    pub fn open(name: *const c_char) -> Option<Self> {
        let handle = unsafe { dlopen(name, 1) };

        Some(Self {
            handle: NonNull::new(handle)?,
        })
    }

    /// Load a symbol of type `T` from the library.
    pub fn load<T>(&self, symbol: *const c_char) -> Option<T> {
        let ptr = unsafe { dlsym(self.handle.as_ptr(), symbol) };

        if ptr.is_null() {
            return None;
        }

        Some(unsafe { mem::transmute_copy(&ptr) })
    }
}

/// Opens `OpenGL.framework`, so OpenGL functions can be loaded.
pub fn load_opengl() -> Library {
    const PATH: &[u8] = b"/System/Library/Frameworks/OpenGL.framework/Versions/Current/OpenGL\0";

    Library::open(PATH.as_ptr().cast()).unwrap()
}
