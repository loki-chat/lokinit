#[macro_export]
macro_rules! cstring {
    ($str:literal) => {
        concat!($str, '\0').as_ptr() as *const std::ffi::c_char
    };
    ($($str:tt)*) => {
        concat!(stringify!($($str)*), '\0').as_ptr() as *const std::ffi::c_char
    };
}

#[macro_export]
macro_rules! class {
    ($name:ident) => {
        unsafe { ffi::objc_getClass(cstring!($name)) }
    };
}

#[macro_export]
macro_rules! sel {
    ($name:literal) => {
        unsafe { ffi::sel_getUid(cstring!($name)) }
    };
}

/// An Objective-C message.
#[macro_export]
macro_rules! msg {
    ($class:ident $sel:ident) => {
        unsafe {
            #[allow(unused_unsafe)]
            let func: extern "C" fn(instance: *mut c_void, msg: *mut c_void) = std::mem::transmute(ffi::objc_msgSend as *const c_void);
            func($class, $sel);
        }
    };
    ($class:ident $sel:ident $($arg_name:ident:$arg:expr)*) => {
        unsafe {
            #[allow(unused_unsafe)]
            let func: extern "C" fn(instance: *mut c_void, msg: *mut c_void$(, $arg_name: _)*) = std::mem::transmute(ffi::objc_msgSend as *const c_void);
            func($class, $sel, $($arg,)*);
        }
    };
}

/// An Objective-C message that returns a value.
#[macro_export]
macro_rules! msg_ret {
    ($class:ident $sel:ident) => {
        unsafe {
            #[allow(unused_unsafe)]
            let func: extern "C" fn(instance: *mut c_void, msg: *mut c_void) -> _ =
                std::mem::transmute(ffi::objc_msgSend as *const c_void);
            func($class, $sel)
        }
    };
    ($class:ident $sel:ident $($arg_name:ident:$arg:expr)*) => {
        unsafe {
            #[allow(unused_unsafe)]
            let func: extern "C" fn(instance: *mut c_void, msg: *mut c_void$(, $arg_name: _)*) -> _ = std::mem::transmute(ffi::objc_msgSend as *const c_void);
            func($class, $sel, $($arg,)*)
        }
    };
}

#[macro_export]
macro_rules! composable_enum {
    (@$type:ty: $name:ident $($variant:ident = $value:expr),*) => {
        pub struct $name(pub $type);
        #[allow(non_upper_case_globals)]
        impl $name {
            $(pub const $variant: Self = Self($value);)*
        }
        impl std::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }
    };
}

pub use {class, composable_enum, cstring, msg, msg_ret, sel};
