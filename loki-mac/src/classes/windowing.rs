use super::classes_prelude::*;

#[objrs]
mod ffi {
    use super::*;

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
        pub fn contains(&self, x: f64, y: f64) -> bool {
            (x >= self.origin.x)
                && (x <= self.origin.x + self.size.width)
                && (y >= self.origin.y)
                && (y <= self.origin.y + self.size.height)
        }
    }

    extern "objc" {
        type NSWindow;

        fn alloc() -> NonNull<NSWindowInstance>;
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
        #[selector = "makeKeyWindow"]
        fn make_key(&mut self);
        #[selector = "makeMainWindow"]
        fn make_main(&mut self);

        #[selector = "setTitle:"]
        fn set_title(&mut self, title: ObjcRc<NSString>);
        fn center(&mut self);
        fn close(&self);

        #[selector = "contentView"]
        fn content_view(&self) -> ObjcRc<NSView>;
        #[selector = "setContentView"]
        fn set_content_view(&mut self, view: ObjcRc<NSView>);
    }
    impl NSWindow {
        pub fn new(size: NSRect, style: NSWindowStyleMask) -> ObjcRc<Self> {
            let instance = NSWindow::alloc();
            let mut window = unsafe { NSWindow::from_raw(instance) };
            window.init(size, style, NSBackingStoreType::Buffered, false.into());

            window
        }

        pub fn focus(&mut self) {
            self.make_key_and_order_front(ptr::null_mut());
            self.make_main();
        }
    }

    extern "objc" {
        type NSView;

        fn alloc() -> NonNull<NSViewInstance>;
        #[selector = "initWithFrame:"]
        fn init(&mut self, size: NSRect);
    }
    impl NSView {
        pub fn new(size: NSRect) -> ObjcRc<Self> {
            let mut this = unsafe { Self::from_raw(Self::alloc()) };
            this.init(size);

            this
        }
    }
}
pub use ffi::*;
