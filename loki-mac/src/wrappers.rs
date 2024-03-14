//! Thin wrapper types for all of the Objective-C classes in the [`crate::ffi`] module.
//! These just provide convenience methods or convert ObjC<->Rust types.

use {
    crate::{enums, ffi},
    core::ptr,
    std::{
        ops::{Deref, DerefMut},
        ptr::NonNull,
    },
};

#[repr(transparent)]
pub struct NSEvent(ffi::NSEvent);
impl NSEvent {
    #[inline(always)]
    pub fn event_subtype(&self) -> enums::NSEventSubtype {
        self.0.event_subtype().into()
    }

    #[inline(always)]
    pub fn mouse_location() -> ffi::NSPoint {
        ffi::NSEvent::mouse_location()
    }
}
impl Deref for NSEvent {
    type Target = ffi::NSEvent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NSEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(transparent)]
pub struct NSApp(ffi::NSApplication);
impl NSApp {
    pub fn shared() -> Self {
        let shared = ffi::NSApplication::shared();
        unsafe { Self(ffi::NSApplication::from_raw(NonNull::new(shared).unwrap())) }
    }

    #[inline(always)]
    pub fn activate(&mut self) {
        // TODO: Detect macOS version and call activate() on 10.14+
        self.0.activate_old(true.into());
    }
    pub fn next_event(
        &mut self,
        mask: enums::NSEventMask,
        until_date: *mut ffi::NSDateInstance,
        mode: enums::NSRunLoopMode,
        dequeue: bool,
    ) -> NSEvent {
        let event_ptr = self
            .0
            .next_event(mask, until_date, mode.into(), dequeue.into());
        let event = unsafe { ffi::NSEvent::from_raw(NonNull::new(event_ptr).unwrap()) };
        NSEvent(event)
    }
    #[inline(always)]
    pub fn send_event(&mut self, event: NSEvent) {
        self.0.send_event(event.into_raw().as_ptr())
    }
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.0.is_active().into()
    }
}
impl Deref for NSApp {
    type Target = ffi::NSApplication;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NSApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(transparent)]
pub struct NSWindow(ffi::NSWindow);
impl NSWindow {
    pub fn new(size: ffi::NSRect, style: enums::NSWindowStyleMask) -> Self {
        let instance = ffi::NSWindow::alloc();
        let mut window = unsafe { ffi::NSWindow::from_raw(NonNull::new(instance).unwrap()) };
        window.init(
            size,
            style,
            enums::NSBackingStoreType::Buffered,
            false.into(),
        );

        Self(window)
    }

    pub fn set_title(&mut self, title: &str) {
        let title: ffi::NSString = title.into();
        self.0.set_title(title.into_raw().as_ptr())
    }

    #[inline(always)]
    pub fn make_main(&mut self) {
        self.0.make_key_and_order_front(ptr::null_mut());
        self.0.make_main();
    }
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        self.0.is_main().into()
    }
}
impl Deref for NSWindow {
    type Target = ffi::NSWindow;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NSWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
