use {super::macros::*, std::ffi::c_void};

/// Pointers to the classes and selectors Lokinit needs.
#[derive(Default, Debug)]
pub struct VTables {
    pub nsapp: NSAppVTable,
    pub nswindow: NSWindowVTable,
    pub nsrect: NSRectVTable,
    pub nsscreen: NSScreenVTable,
    pub nsstring: NSStringVTable,
    pub nsdate: NSDateVTable,
    pub nsevent: NSEventVTable,
    pub nscursor: NSCursorVTable,
    pub nsimage: NSImageVTable,
    pub nsbutton: NSButtonVTable,
}

/// A VTable for the NSApplication class.
#[derive(Debug)]
pub struct NSAppVTable {
    /// The NSApplication class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The NSApplication instance returned from [NSApplication sharedApplication].
    pub shared: *mut c_void,
    /// The [NSApplication nextEvent] message.
    pub next_event_sel: *mut c_void,
    /// The [NSApplication setActivationPolicy:] message.
    pub set_activation_policy_sel: *mut c_void,
    // TODO: On macOS >= 10.14, this method is deprecated, and [NSApplication activate] is used instead.
    /// The [NSApplication activateIgnoringOtherApps:] message.
    pub activate_ignoring_other_apps_sel: *mut c_void,
    /// The [NSApplication finishLaunching] message.
    pub finish_launching_sel: *mut c_void,
    /// The [NSApplication nextEventMatchingMask:untilDate:inMode:dequeue:] message.
    pub next_event_matching_sel: *mut c_void,
    /// The [NSApplication sendEvent:] message.
    pub send_event_sel: *mut c_void,
}
impl Default for NSAppVTable {
    fn default() -> Self {
        let nsapp = class!(NSApplication);
        let shared_application = sel!("sharedApplication");
        let shared: *mut c_void = msg_ret![nsapp shared_application];
        assert!(!shared.is_null());

        Self {
            class: nsapp,
            shared,
            next_event_sel: sel!("nextEvent"),
            set_activation_policy_sel: sel!("setActivationPolicy:"),
            activate_ignoring_other_apps_sel: sel!("activateIgnoringOtherApps:"),
            finish_launching_sel: sel!("finishLaunching"),
            next_event_matching_sel: sel!("nextEventMatchingMask:untilDate:inMode:dequeue:"),
            send_event_sel: sel!("sendEvent:"),
        }
    }
}

/// A VTable for the NSWindow class.
#[derive(Debug)]
pub struct NSWindowVTable {
    /// The NSWindow class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The [NSWindow alloc] message.
    pub alloc_sel: *mut c_void,
    /// The [NSWindow initWithContentRect:styleMask:backing:defer:screen:] message.
    pub constructor_sel: *mut c_void,
    /// The [NSWindow makeKeyAndOrderFront:] message.
    pub make_key_and_order_front_sel: *mut c_void,
    /// The [NSWindow makeMainWindow] message.
    pub make_main_window_sel: *mut c_void,
    /// The [NSWindow setTitle:] message.
    pub set_title_sel: *mut c_void,
    /// The [NSWindow center] message.
    pub center_sel: *mut c_void,
    /// The [NSWindow windowNumber] message.
    pub window_number_sel: *mut c_void,
    /// The [NSWindow frame] message.
    pub frame_sel: *mut c_void,
    /// The [NSWindow setFrame:display:] message.
    pub set_frame_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nswindow/1419491-standardwindowbutton?language=objc
    pub std_window_btn_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nswindow/1419639-disablecursorrects?language=objc
    pub disable_cursor_rects_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nswindow/1419662-close?language=objc
    pub close_sel: *mut c_void,
}
impl Default for NSWindowVTable {
    fn default() -> Self {
        Self {
            class: class!(NSWindow),
            alloc_sel: sel!("alloc"),
            constructor_sel: sel!("initWithContentRect:styleMask:backing:defer:screen:"),
            make_key_and_order_front_sel: sel!("makeKeyAndOrderFront:"),
            make_main_window_sel: sel!("makeMainWindow"),
            set_title_sel: sel!("setTitle:"),
            center_sel: sel!("center"),
            window_number_sel: sel!("windowNumber"),
            frame_sel: sel!("frame"),
            set_frame_sel: sel!("setFrame:display:"),
            std_window_btn_sel: sel!("standardWindowButton:"),
            disable_cursor_rects_sel: sel!("disableCursorRects"),
            close_sel: sel!("close"),
        }
    }
}

/// A VTable for the NSButton class.
#[derive(Debug)]
pub struct NSButtonVTable {
    /// https://developer.apple.com/documentation/appkit/nsbutton/1534156-highlight?language=objc
    pub highlight_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsview/1483737-superview?language=objc
    /// Ok, yes, it's from NSView, but I'm too lazy to make a new vtable for just this
    pub superview_sel: *mut c_void,
}
impl Default for NSButtonVTable {
    fn default() -> Self {
        Self {
            highlight_sel: sel!("highlight:"),
            superview_sel: sel!("superview"),
        }
    }
}

/// A VTable for the NSRect class.
#[derive(Debug)]
pub struct NSRectVTable {
    /// The NSRect class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The [NSRect alloc] message.
    pub alloc_sel: *mut c_void,
    /// The [NSRect x:y:width:height:] message.
    pub constructor_sel: *mut c_void,
}
impl Default for NSRectVTable {
    fn default() -> Self {
        Self {
            class: class!(NSRect),
            alloc_sel: sel!("alloc"),
            constructor_sel: sel!("x:y:width:height:"),
        }
    }
}

/// A VTable for the NSScreen class.
#[derive(Debug)]
pub struct NSScreenVTable {
    /// The NSScreen class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The [NSScreen mainScreen] message.
    pub main_screen_sel: *mut c_void,
}
impl Default for NSScreenVTable {
    fn default() -> Self {
        Self {
            class: class!(NSScreen),
            main_screen_sel: sel!("mainScreen"),
        }
    }
}

/// A VTable for the NSString class.
#[derive(Debug)]
pub struct NSStringVTable {
    /// The NSString class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The [NSString alloc] message.
    pub alloc: *mut c_void,
    /// The [NSString initWithBytes:length:encoding] message.
    pub init_with_bytes_length_encoding_sel: *mut c_void,
}
impl Default for NSStringVTable {
    fn default() -> Self {
        Self {
            class: class!(NSString),
            alloc: sel!("alloc"),
            init_with_bytes_length_encoding_sel: sel!("initWithBytes:length:encoding:"),
        }
    }
}

/// A VTable for the NSDate class.
#[derive(Debug)]
pub struct NSDateVTable {
    /// The NSDate for a time in the distant future.
    pub distant_future: *mut c_void,
}
impl Default for NSDateVTable {
    fn default() -> Self {
        let nsdate = class!(NSDate);
        let distant_future = sel!("distantFuture");
        let distant_future = msg_ret![nsdate distant_future];

        Self { distant_future }
    }
}

/// A VTable for the NSEvent class.
#[derive(Debug)]
pub struct NSEventVTable {
    /// The NSEvent class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The [NSEvent type] message.
    pub type_sel: *mut c_void,
    /// The [NSEvent subtype] message.
    pub subtype_sel: *mut c_void,
    /// The [NSEvent windowNumber] message.
    pub window_number_sel: *mut c_void,
    /// The [NSEvent mouseLocation] message.
    pub mouse_location_sel: *mut c_void,
    /// https://developer.apple.com/documentation/appkit/nsevent/1527828-buttonnumber?language=objc
    pub button_number_sel: *mut c_void,
}
impl Default for NSEventVTable {
    fn default() -> Self {
        Self {
            class: class!(NSEvent),
            type_sel: sel!("type"),
            subtype_sel: sel!("subtype"),
            window_number_sel: sel!("windowNumber"),
            mouse_location_sel: sel!("mouseLocation"),
            button_number_sel: sel!("buttonNumber"),
        }
    }
}

/// A VTable for the NSCursor class.
#[derive(Debug)]
pub struct NSCursorVTable {
    // TODO: Add docs for these ptrs
    pub class: *mut c_void,
    pub alloc: *mut c_void,
    /// The [NSCursor initWithImage:hotSpot:] message.
    pub init_sel: *mut c_void,
    pub set_sel: *mut c_void,
}
impl Default for NSCursorVTable {
    fn default() -> Self {
        Self {
            class: class!(NSCursor),
            alloc: sel!("alloc"),
            init_sel: sel!("initWithImage:hotSpot:"),
            set_sel: sel!("set"),
        }
    }
}

/// A VTable for the NSImage class.
#[derive(Debug)]
pub struct NSImageVTable {
    /// The NSImage class pointer.
    pub class: *mut c_void,
    // TODO: Should link to apple docs instead of listing useless objc methods
    /// The [NSImage alloc] message.
    pub alloc: *mut c_void,
    /// The [initWithContentsOfFile:] message.
    pub init_sel: *mut c_void,
}
impl Default for NSImageVTable {
    fn default() -> Self {
        Self {
            class: class!(NSImage),
            alloc: sel!("alloc"),
            init_sel: sel!("initWithContentsOfFile:"),
        }
    }
}
