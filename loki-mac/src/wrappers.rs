//! Thin wrapper types for all of the Objective-C classes in the [`crate::ffi`] module.
//! These just provide convenience methods or convert ObjC<->Rust types.

// #[repr(transparent)]
// pub struct NSApp(ffi::NSApplication);
// impl NSApp {
//     #[inline(always)]
//     pub fn activate(&mut self) {
//         // TODO: Detect macOS version and call activate() on 10.14+
//         self.0.activate_old(true.into());
//     }
//     pub fn next_event(
//         &mut self,
//         mask: enums::NSEventMask,
//         until_date: ffi::NSDate,
//         mode: enums::NSRunLoopMode,
//         dequeue: bool,
//     ) -> Option<ffi::NSEvent> {
//         let event_ptr = self
//             .0
//             .next_event(mask, until_date, mode.into(), dequeue.into());
//         let event = unsafe { ffi::NSEvent::from_raw(NonNull::new(event_ptr)?) };
//         Some(event)
//     }
//     #[inline(always)]
//     pub fn is_active(&self) -> bool {
//         self.0.is_active().into()
//     }
// }
// impl Deref for NSApp {
//     type Target = ffi::NSApplication;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for NSApp {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// #[repr(transparent)]
// pub struct NSWindow(ffi::NSWindow);
// impl NSWindow {
//     pub fn new(size: ffi::NSRect, style: enums::NSWindowStyleMask) -> Self {
//         let instance = ffi::NSWindow::alloc();
//         let mut window = unsafe { ffi::NSWindow::from_raw(NonNull::new(instance).unwrap()) };
//         window.init(
//             size,
//             style,
//             enums::NSBackingStoreType::Buffered,
//             false.into(),
//         );

//         Self(window)
//     }

//     pub fn set_title(&mut self, title: &str) {
//         // TODO: Debug why windows crash when setting an empty title
//         // I can't tell if it's an issue with how we make the NSString or it just
//         // doesn't accept empty strings. Either way it exits with an error
//         // suggesting we forgot to nest alloc and init.
//         let title: ffi::NSString = if title.len() > 0 {
//             title.into()
//         } else {
//             " ".into()
//         };
//         self.0.set_title(title.into_raw().as_ptr())
//     }

//     #[inline(always)]
//     pub fn make_main(&mut self) {
//         // self.0.make_key_and_order_front(ptr::null_mut());
//         // self.0.make_key();
//         self.0.make_main();
//     }
//     #[inline(always)]
//     pub fn is_main(&self) -> bool {
//         self.0.is_main().into()
//     }
// }
// impl Deref for NSWindow {
//     type Target = ffi::NSWindow;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for NSWindow {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
