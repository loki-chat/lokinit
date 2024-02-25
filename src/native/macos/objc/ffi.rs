use std::ffi::{c_char, c_void};

#[link(name = "objc")]
extern "C" {
    /// This isn't the function signature. The function gets cast to the correct type when we call
    /// it because variadic functions don't work the way it needs.
    pub fn objc_msgSend(instance: *mut c_void, msg: *mut c_void) -> *mut c_void;
    pub fn objc_getClass(name: *const c_char) -> *mut c_void;
    pub fn sel_getUid(name: *const c_char) -> *mut c_void;
    pub fn class_getClassVariable(class: *mut c_void, name: *const c_char) -> *mut c_void;
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub static NSDefaultRunLoopMode: *mut c_void;
}

// Yes, this link statement is empty, and yes, it's supposed to be here.
// Without linking to AppKit, Objective-C can't find AppKit's classes,
// eg NSApplication or NSWindow etc.
//
// What's really wild is that Objective-C won't error if it can't find the class.
// It just returns null, and if you send a message to that class, it fails silently.
#[link(name = "AppKit", kind = "framework")]
extern "C" {}
