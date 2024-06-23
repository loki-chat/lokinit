use super::classes_prelude::*;

#[objrs]
mod ffi {
    use {super::*, core::ffi::c_char, std::ffi::CString};

    extern "objc" {
        type NSApplication;

        #[selector = "sharedApplication"]
        fn shared() -> NSApplication;

        #[selector = "nextEventMatchingMask:untilDate:inMode:dequeue:"]
        fn next_event(
            &self,
            mask: NSEventMask,
            until_date: NSDate,
            mode: NSRunLoopMode,
            dequeue: ObjcBool,
        ) -> Option<NSEvent>;
        #[selector = "sendEvent:"]
        fn send_event(&mut self, event: NSEvent);

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
    impl NSApplication {}

    extern "objc" {
        type NSEvent;

        #[selector = "type"]
        fn event_type(&self) -> NSEventType;
        #[selector = "subtype"]
        fn event_subtype_raw(&self) -> i16;
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
        fn characters_raw(&self) -> ObjcRc<NSString>;
    }
    impl NSEvent {
        pub fn characters(&self) -> Option<&str> {
            let nsstring = self.characters_raw();

            // Safety: The string will be valid for as long as this event exists... so as long as there's
            // not another issue with this NSEvent instance, this is safe.
            unsafe { Some(nsstring.as_str()) }
        }

        #[inline(always)]
        pub fn event_subtype(&self) -> NSEventSubtype {
            self.event_subtype_raw().into()
        }
    }

    extern "objc" {
        type NSString;

        #[selector = "stringWithUTF8String:"]
        fn new_with_string(string: *const c_char) -> Option<ObjcRc<Self>>;
        #[selector = "cStringUsingEncoding:"]
        fn to_c_string(&self, encoding: NSStringEncoding) -> *const c_char;
        #[selector = "lengthOfBytesUsingEncoding:"]
        fn length_of_bytes(&self, encoding: NSStringEncoding) -> usize;
    }
    impl NSString {
        pub fn from_str<S: AsRef<str>>(str: S) -> ObjcRc<Self> {
            let string = CString::new(str.as_ref()).unwrap();
            let instance = NSString::new_with_string(string.as_ptr());
            instance.unwrap()
        }

        /// # Safety
        /// The returned string must not be borrowed for longer than this NSString exists - in other
        /// words, `'a` must not outlive this string.
        pub unsafe fn as_str<'a>(&self) -> &'a str {
            let c_string: *const u8 = self.to_c_string(NSStringEncoding::UTF8).cast();
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

    extern "objc" {
        type NSDate;

        #[selector = "distantFuture"]
        fn distant_future() -> Self;
        #[selector = "distantPast"]
        fn distant_past() -> Self;
        fn now() -> Self;
    }

    /// https://developer.apple.com/documentation/foundation/nsrunloopmode?language=objc
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct NSRunLoopMode(NonNull<NSStringInstance>);
    impl NSRunLoopMode {
        pub fn default() -> Self {
            unsafe { crate::_link::NSDefaultRunLoopMode }
        }
        pub fn common() -> Self {
            unsafe { crate::_link::NSRunLoopCommonModes }
        }
        pub fn modal_panel() -> Self {
            unsafe { crate::_link::NSModalPanelRunLoopMode }
        }
        pub fn event_tracking() -> Self {
            unsafe { crate::_link::NSEventTrackingRunLoopMode }
        }
    }
}
pub use ffi::*;
