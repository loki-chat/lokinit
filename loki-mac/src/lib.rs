pub mod dynload;
pub mod enums;
pub mod wrappers;

use objective_rust::objrs;
pub use {
    enums::*,
    ffi::{
        NSDate, NSOpenGLContext, NSOpenGLPixelFormat, NSOpenGLView, NSPoint, NSRect, NSSize, NSView,
    },
    wrappers::*,
};

#[objrs]
pub mod ffi {
    use {
        crate::enums::*,
        objective_rust::ObjcBool,
        std::{
            ffi::{c_char, CString},
            ptr::{self, NonNull},
        },
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
    impl NSRect {
        pub fn contains(&self, x: i32, y: i32) -> bool {
            (x >= self.origin.x as i32)
                && (x <= self.origin.x as i32 + self.size.width as i32)
                && (y >= self.origin.y as i32)
                && (y <= self.origin.y as i32 + self.size.height as i32)
        }
    }

    extern "objc" {
        type NSString;

        #[selector = "stringWithUTF8String:"]
        fn new_with_string(string: *const c_char) -> *mut Self;
        #[selector = "cStringUsingEncoding:"]
        fn to_c_string(&self, encoding: NSStringEncoding) -> *const c_char;
        #[selector = "lengthOfBytesUsingEncoding:"]
        fn length_of_bytes(&self, encoding: NSStringEncoding) -> usize;
    }
    impl From<String> for NSString {
        fn from(value: String) -> Self {
            Self::from(value.as_str())
        }
    }
    impl From<&str> for NSString {
        fn from(value: &str) -> Self {
            let string = CString::new(value).unwrap();
            let instance = NSString::new_with_string(string.as_ptr());

            unsafe { NSString::from_raw(NonNull::new(instance).unwrap()) }
        }
    }
    impl NSString {
        /// # Safety
        /// The returned string must not be borrowed for longer than this NSString exists - in other
        /// words, `'a` must not outlive this string.
        pub unsafe fn as_str<'a>(&self) -> &'a str {
            let c_string: *const u8 = self.to_c_string(crate::NSStringEncoding::UTF8).cast();
            let len = self.len();
            let buffer = std::slice::from_raw_parts(c_string, len);

            unsafe { std::str::from_utf8_unchecked(buffer) }
        }

        #[inline(always)]
        pub fn len(&self) -> usize {
            self.length_of_bytes(NSStringEncoding::UTF8)
        }
        #[inline(always)]
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
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

        #[selector = "contentView"]
        fn content_view(&self) -> NSView;
        #[selector = "setContentView"]
        fn set_content_view(&mut self, view: *mut NSViewInstance);
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
        #[selector = "buttonNumber"]
        fn mouse_button_number(&self) -> isize;
        #[selector = "characters"]
        fn characters(&self) -> *const NSStringInstance;
    }

    extern "objc" {
        type NSView;

        fn alloc() -> *mut Self;
        #[selector = "initWithFrame:"]
        fn init(&mut self, size: NSRect);
    }
    impl NSView {
        pub fn new(size: NSRect) -> Self {
            let mut this = unsafe { Self::from_raw(NonNull::new(Self::alloc()).unwrap()) };
            this.init(size);

            this
        }
    }

    extern "objc" {
        type NSOpenGLView;

        fn alloc() -> *mut Self;
        #[selector = "defaultPixelFormat"]
        fn default_pixel_format() -> NSOpenGLPixelFormat;
        #[selector = "initWithFrame:pixelFormat:"]
        fn init(&mut self, frame: NSRect, pixel_format: NSOpenGLPixelFormat);
    }

    extern "objc" {
        type NSOpenGLPixelFormat;

        fn alloc() -> *mut Self;
        #[selector = "initWithAttributes:"]
        fn init(&mut self, attributes: *const u32);
        #[selector = "numberOfVirtualScreens"]
        fn number_of_virtual_screens(&self) -> i32;
    }
    impl NSOpenGLPixelFormat {
        pub fn new(attributes: &[u32]) -> Self {
            let mut this = unsafe { Self::from_raw(NonNull::new(Self::alloc()).unwrap()) };
            this.init(attributes as *const [u32] as *const u32);

            this
        }
    }

    extern "objc" {
        type NSOpenGLContext;

        fn alloc() -> *mut Self;
        #[selector = "initWithFormat:shareContext:"]
        fn init(
            &mut self,
            format: NSOpenGLPixelFormat,
            share_context: *mut NSOpenGLContextInstance,
        );
        #[selector = "setView:"]
        fn set_view(&mut self, view: *mut NSViewInstance);
        #[selector = "makeCurrentContext"]
        fn make_current(&self);
        #[selector = "setFullScreen"]
        fn set_fullscreen(&self);
        fn update(&self);
        #[selector = "flushBuffer"]
        fn flush_buffer(&self);
    }
    impl NSOpenGLContext {
        pub fn new(format: NSOpenGLPixelFormat) -> Self {
            let mut this = unsafe { Self::from_raw(NonNull::new(Self::alloc()).unwrap()) };
            this.init(format, ptr::null_mut());

            this
        }
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
    #[link(name = "Foundation", kind = "framework")]
    extern "C" {}
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {}
    #[link(name = "Metal", kind = "framework")]
    extern "C" {}
    #[link(name = "system")]
    extern "C" {}
}
