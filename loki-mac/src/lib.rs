use {
    objective_rust::objrs,
    std::{ffi::CString, ptr::NonNull},
};

pub mod enums;
pub mod wrappers;

pub use {
    enums::*,
    ffi::{NSDate, NSPoint, NSRect, NSSize},
    wrappers::*,
};

#[objrs]
pub mod ffi {
    use {
        crate::enums::*,
        objective_rust::ObjcBool,
        std::{ffi::c_char, ptr::NonNull},
    };

    #[repr(C)]
    #[derive(Debug, Clone, Default)]
    pub struct NSPoint {
        pub x: f64,
        pub y: f64,
    }
    #[repr(C)]
    #[derive(Debug, Clone, Default)]
    pub struct NSSize {
        pub width: f64,
        pub height: f64,
    }
    #[repr(C)]
    #[derive(Debug, Clone, Default)]
    pub struct NSRect {
        pub origin: NSPoint,
        pub size: NSSize,
    }

    extern "objc" {
        type NSString;

        #[selector = "stringWithUTF8String:"]
        fn new(string: *const c_char) -> *mut Self;
    }

    /// https://developer.apple.com/documentation/foundation/nsrunloopmode?language=objc
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct NSRunLoopMode(NonNull<()>);

    extern "objc" {
        type NSDate;

        #[selector = "distantFuture"]
        fn distant_future() -> *mut Self;
        #[selector = "distantPast"]
        fn distant_past() -> *mut Self;
    }

    extern "objc" {
        type NSApplication;

        #[selector = "sharedApplication"]
        fn shared() -> *mut Self;

        #[selector = "nextEventMatchingMask:untilDate:inMode:dequeue:"]
        fn next_event(
            &mut self,
            mask: NSEventMask,
            until_date: *mut NSDateInstance,
            mode: NSRunLoopMode,
            dequeue: ObjcBool,
        ) -> *mut NSEventInstance;
        #[selector = "sendEvent:"]
        fn send_event(&mut self, event: *mut NSEventInstance);

        #[selector = "activateIgnoringOtherApps:"]
        fn activate_old(&mut self, ignore_other_apps: ObjcBool);
        // TODO: This is the new method for activating an app. Should use it
        // instead of `activate_old` on macOS 14+.
        #[selector = "activate"]
        fn activate(&mut self);
        #[selector = "finishLaunching"]
        fn finish_launching(&mut self);
        #[selector = "setActivationPolicy:"]
        fn set_activation_policy(&mut self, policy: NSApplicationActivationPolicy);
        #[selector = "isActive"]
        fn is_active(&self) -> ObjcBool;

        fn run(&mut self);
    }

    extern "objc" {
        type NSWindow;

        fn alloc() -> *mut Self;
        #[selector = "initWithContentRect:styleMask:backing:defer:"]
        fn init(
            &mut self,
            content_rect: NSRect,
            style_mask: NSWindowStyleMask,
            backing_store: NSBackingStoreType,
            defer: ObjcBool,
        );

        #[selector = "windowNumber"]
        fn id(&self) -> isize;
        fn frame(&self) -> NSRect;
        #[selector = "setFrame:display:animate:"]
        fn set_frame(&mut self, frame: NSRect, display: ObjcBool, animate: ObjcBool);
        #[selector = "mouseLocationOutsideOfEventStream"]
        fn mouse_location(&self) -> NSPoint;
        #[selector = "isMainWindow"]
        fn is_main(&self) -> ObjcBool;

        #[selector = "makeKeyAndOrderFront:"]
        fn make_key_and_order_front(&mut self, sender: *mut ());
        #[selector = "makeMainWindow"]
        fn make_main(&mut self);

        #[selector = "setTitle:"]
        fn set_title(&mut self, title: *mut NSStringInstance);
        fn center(&mut self);
        fn close(&self);
    }

    extern "objc" {
        type NSEvent;

        #[selector = "type"]
        fn event_type(&self) -> NSEventType;
        #[selector = "subtype"]
        fn event_subtype(&self) -> i16;
        #[selector = "keyCode"]
        fn key_code(&self) -> u32;
        #[selector = "windowNumber"]
        fn window_number(&self) -> isize;
        #[selector = "mouseLocation"]
        fn mouse_location() -> NSPoint;
        #[selector = "isARepeat"]
        fn is_repeat(&self) -> ObjcBool;
    }

    // Without this, Rust won't link to AppKit and AppKit classes won't get loaded.
    #[link(name = "AppKit", kind = "framework")]
    extern "C" {
        /// https://developer.apple.com/documentation/foundation/nsrunloopcommonmodes?language=objc
        pub static NSRunLoopCommonModes: NSRunLoopMode;
        /// https://developer.apple.com/documentation/foundation/nsdefaultrunloopmode?language=objc
        pub static NSDefaultRunLoopMode: NSRunLoopMode;
        /// https://developer.apple.com/documentation/appkit/nseventtrackingrunloopmode?language=objc
        pub static NSEventTrackingRunLoopMode: NSRunLoopMode;
        /// https://developer.apple.com/documentation/appkit/nsmodalpanelrunloopmode?language=objc
        pub static NSModalPanelRunLoopMode: NSRunLoopMode;
    }
}

impl From<String> for ffi::NSString {
    fn from(value: String) -> Self {
        let string = CString::new(value).unwrap();
        let instance = ffi::NSString::new(string.as_ptr());

        unsafe { ffi::NSString::from_raw(NonNull::new(instance).unwrap()) }
    }
}
impl From<&str> for ffi::NSString {
    fn from(value: &str) -> Self {
        let string = CString::new(value).unwrap();
        let instance = ffi::NSString::new(string.as_ptr());

        unsafe { ffi::NSString::from_raw(NonNull::new(instance).unwrap()) }
    }
}

impl ffi::NSRect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        (x >= self.origin.x as i32)
            && (x <= self.origin.x as i32 + self.size.width as i32)
            && (y >= self.origin.y as i32)
            && (y <= self.origin.y as i32 + self.size.height as i32)
    }
}
