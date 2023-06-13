use std::ffi::{c_void, CString};
use std::ptr::NonNull;

use self::dl::{dlopen, dlsym, get_dlerror, RTLD_NOW};

mod dl;
mod locale;
pub mod wayland;
pub mod x11;

/// A wrapper for all the dl methods
struct Library {
    handle: NonNull<c_void>,
}

impl Library {
    /// Create a new Library from the name of its file.
    ///
    /// Usually, Linux libraries are in the form `lib<name>.so`.
    ///
    /// This function *will* append the `lib` prefix and `.so` extension for you, so don't do it yourself.
    pub fn new(name: &str) -> Result<Self, String> {
        let c_str = CString::new(format!("lib{}.so", name)).unwrap();
        let lib = unsafe { dlopen(c_str.as_ptr(), RTLD_NOW) };

        if lib.is_null() {
            Err(get_dlerror().unwrap())
        } else {
            Ok(Self {
                handle: unsafe { NonNull::new_unchecked(lib) },
            })
        }
    }

    pub fn get_sym<F: Sized>(&self, sym: &str) -> Result<F, String> {
        let c_str = CString::new(sym).unwrap();
        let sym = unsafe { dlsym(self.handle.as_ptr(), c_str.as_ptr()) };

        if sym.is_null() {
            Err(get_dlerror().unwrap())
        } else {
            Ok(unsafe { std::mem::transmute_copy::<_, F>(&sym) })
        }
    }
}

#[derive(Clone, Debug)]
pub enum LoadingError {
    Library(String),
    Symbol(String),
}

/// Generates a library struct that loads all the specified functions from the library at runtime.
#[macro_export]
macro_rules! library {
    (
        [ $lib:ident <-> $name:literal ] ;
        $( pub fn $fn:ident $args:tt $(-> $res:ty)? );* ;
    ) => {
        pub struct $lib {
            $(pub $fn: unsafe extern "C" fn$args$( -> $res)?,)*
        }

        impl $lib {
            pub unsafe fn new() -> Result<Self, $crate::native::linux::LoadingError> {
                let lib = $crate::native::linux::Library::new($name)
                    .map_err($crate::native::linux::LoadingError::Library)?;

                Ok(Self {
                    $($fn: lib.get_sym(stringify!($fn)).map_err($crate::native::linux::LoadingError::Symbol)?,)*
                })
            }
        }
    };
}