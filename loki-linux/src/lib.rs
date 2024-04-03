use std::error::Error;
use std::ffi::{c_void, CString};
use std::fmt;
use std::ptr::NonNull;
use std::rc::Rc;

use dl::{dlopen, dlsym, get_dlerror, RTLD_NOW};

pub mod dl;
pub mod locale;
pub mod wayland;
pub mod x11;

/// A wrapper for all the dl methods
pub struct Library {
    handle: NonNull<c_void>,
}

impl Library {
    /// Create a new Library from the name of its file.
    ///
    /// Usually, Linux libraries are in the form `lib<name>.so`.
    ///
    /// This function *will* append the `lib` prefix and `.so` extension for you, so don't do it yourself.
    pub fn new(name: &str) -> Result<Self, LoadingError> {
        let c_str = CString::new(format!("lib{}.so", name)).unwrap();
        let lib = unsafe { dlopen(c_str.as_ptr(), RTLD_NOW) };

        if lib.is_null() {
            Err(LoadingError::Library(get_dlerror().unwrap()))
        } else {
            Ok(Self {
                handle: unsafe { NonNull::new_unchecked(lib) },
            })
        }
    }

    pub fn get_sym<F: Sized>(&self, sym: &str) -> Result<F, LoadingError> {
        let c_str = CString::new(sym).unwrap();
        let sym = unsafe { dlsym(self.handle.as_ptr(), c_str.as_ptr()) };

        if sym.is_null() {
            Err(LoadingError::Symbol(get_dlerror().unwrap()))
        } else {
            Ok(unsafe { std::mem::transmute_copy::<_, F>(&sym) })
        }
    }
}

#[derive(Clone, Debug)]
pub enum LoadingError {
    Library(Rc<str>),
    Symbol(Rc<str>),
}

impl Error for LoadingError {}

impl fmt::Display for LoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadingError::Library(msg) => write!(f, "(Library) {}", msg),
            LoadingError::Symbol(msg) => write!(f, "(Symbol) {}", msg),
        }
    }
}

/// Generates a library struct that loads all the specified functions from the library at runtime.
#[macro_export]
macro_rules! library {
    (
        [ $lib:ident <-> $name:literal ] ;
        $( { $( pub $var:ident : $vartype:ty );* ; } )?
        $( pub fn $fn:ident $args:tt $(-> $res:ty)? );* ;
    ) => {
        pub struct $lib {
            $( $(pub $var: $vartype,)* )?
            $(pub $fn: unsafe extern "C" fn$args$( -> $res)?,)*
        }

        impl $lib {
            /// Instantiates this library.
            ///
            /// # Safety
            ///
            /// This method calls some `dlopen` functions through FFI.
            pub unsafe fn new() -> Result<Self, $crate::LoadingError> {
                let lib = $crate::Library::new($name)?;

                Ok(Self {
                    $( $($var: lib.get_sym(stringify!($var))?,)* )?
                    $($fn: lib.get_sym(stringify!($fn))?,)*
                })
            }
        }
    };
}
