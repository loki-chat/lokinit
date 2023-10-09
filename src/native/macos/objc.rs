//! Bindings for all the Objective-C classes used by Lokinit on macOS. These are used to interact
//! with AppKit.
//!
//! # Implementation Details
//!
//! There's a few C functions that allow other languages to run Objective-C code. This binds to
//! those functions (see `ffi.rs` for code), then uses them to find the classes (and messages for
//! those classes) we need to interact with AppKit.
//!
//! # "Why not just use the `objc` crate?"
//!
//! Thanks for the excellent question, hypothetical docs reader. Initially, Lokinit *did* use the
//! `objc` crate, but we switched to our own implementation. There's a few reasons why we did this:
//!
//! 1. The `objc` crate tries to cache pointers to class/message pointers, but doesn't do it very
//!    well. Our implementation caches pointers to almost every single class and message we need
//!    (look in `vtables.rs` for code), so we don't have to keep asking Objective-C for those
//!    pointers. I'm not sure how much this actually ends up impacting performance, but it's nice
//!    either way.
//! 2. The crate's source code is hard to understand, and has almost no comments explaining how it
//!    works. This means that if Loki ever encountered an issue with the crate, we probably wouldn't
//!    be able to fix it. Maintaining our own implementation gives us a better understanding of how
//!    the code works and gives us absolute control to make changes or fix bugs. We can also now
//!    write that missing documentation ourselves :)
//! 3. The code you need to run Objective-C is actually very minimal. There's a few C functions
//!    that allows C to access Objective-C classes, selectors, variables, etc (see the `ffi` file
//!    in this module). What's more, we don't even need all of those functions, meaning the amount
//!    of code to get this working is actually very minimal (probably < 100 lines if you remove
//!    the VTables).
//! 4. The `objc` crate gives more functionality than we need (for example, creating custom classes,
//!    giving those methods and variables, inheritence...). The crate goes out of the scope of
//!    what we actually need for lokinit.

// Important notes if you're reading this source code:
// 1. Apple's grid systems are based from the *bottom*-left of the screen, not the top-left. Mouse
//    locations are reported relative to the bottom-left corner, and the window's origin is also
//    based on its lower-left corner.

use std::time::Duration;

use crate::prelude::WindowHandle;

use super::MacosBackend;

pub mod cursor;
pub mod enums;
pub mod ffi;
pub mod macros;
pub mod vtables;

use {
    crate::event::*,
    cursor::{Cursors, MacOsCursor},
    enums::*,
    macros::*,
    std::ffi::c_void,
    vtables::VTables,
};

// region: NSApp

pub struct NSApp {}
impl NSApp {
    /// We have to tell NSApplication to launch and configure a few things before we can use it.
    /// This method takes care of that.
    pub fn load() {
        let (instance, set_activation_policy, activate, finish_launching) =
            VTables::with(|vtables| {
                (
                    vtables.nsapp.shared,
                    vtables.nsapp.set_activation_policy_sel,
                    vtables.nsapp.activate_ignoring_other_apps_sel,
                    vtables.nsapp.finish_launching_sel,
                )
            });

        let activation_policy = NSApplicationActivationPolicy::Regular as usize;

        msg![instance set_activation_policy setActivationPolicy:activation_policy];
        msg![instance activate activateIgnoringOtherApps:true];
        msg![instance finish_launching];
    }

    /// Calls [NSApp nextEventMatchingMask:...], then creates an NSEvent struct around that.
    pub fn next_event() -> NSEvent {
        let (nsapp, next_event, distant_future) = VTables::with(|vtables| {
            (
                vtables.nsapp.shared,
                vtables.nsapp.next_event_matching_sel,
                vtables.nsdate.distant_future,
            )
        });
        // Matches all NSEvent masks
        let mask = usize::MAX;
        let mode = unsafe { ffi::NSDefaultRunLoopMode };

        let ptr = msg_ret![nsapp next_event nextEventMatchingMask:mask untilDate:distant_future inMode:mode dequeue:true];
        NSEvent { ptr }
    }

    /// Forwards an event to the NSApplication. Lokinit tries to do this as little as possible,
    /// because this method sometimes hijacks the thread (for example, if you forward resize events
    /// to it, the whole thread will freeze until the resize is completed).
    pub fn send_event(event: NSEvent) {
        let (nsapp, send_event) =
            VTables::with(|vtables| (vtables.nsapp.shared, vtables.nsapp.send_event_sel));
        let event = event.ptr;

        msg![nsapp send_event sendEvent:event];
    }
}

// endregion: NSApp

// region: NSWindow

#[derive(Clone, Copy)]
pub enum NSWindowBorder {
    Top,
    Bottom,
    Left,
    Right,

    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// A struct for interacting with an Objective-C NSWindow instance.
pub struct NSWindow {
    /// A pointer to the actual (Objective-C) NSWindow instance.
    pub ptr: *mut c_void,
    // Mouse targets in the window. If the mouse starts dragging from in one of these borders, we
    // should resize the window.
    pub top_mouse_border: NSRect,
    pub bottom_mouse_border: NSRect,
    pub left_mouse_border: NSRect,
    pub right_mouse_border: NSRect,
    pub top_left_mouse_border: NSRect,
    pub top_right_mouse_border: NSRect,
    pub bottom_left_mouse_border: NSRect,
    pub bottom_right_mouse_border: NSRect,
    /// The direction the window is currently being resized.
    pub resize_direction: Option<NSWindowBorder>,
    /// The window border the mouse is currently hovered over.
    pub hover_border: Option<NSWindowBorder>,
    /// The window size.
    pub rect: NSRect,
    /// The window's ID.
    pub id: usize,
}
impl NSWindow {
    const BORDER_SIZE: f64 = 7.;

    pub fn new(rect: NSRect, centered: bool, name: &str, backend: &mut MacosBackend) -> Self {
        let (nswindow, alloc, init, id, set_title, center, nsscreen, main_screen) =
            VTables::with(|vtables| {
                (
                    vtables.nswindow.class,
                    vtables.nswindow.alloc_sel,
                    vtables.nswindow.constructor_sel,
                    vtables.nswindow.window_number_sel,
                    vtables.nswindow.set_title_sel,
                    vtables.nswindow.center_sel,
                    vtables.nsscreen.class,
                    vtables.nsscreen.main_screen_sel,
                )
            });

        let backing: usize = NSBackingStoreType::Buffered as usize;
        let mask: usize = (NSWindowStyleMask::Titled
            | NSWindowStyleMask::Miniaturizable
            | NSWindowStyleMask::Closable
            | NSWindowStyleMask::Resizable)
            .0;
        let screen: *mut c_void = msg_ret![nsscreen main_screen];

        let instance = msg_ret![nswindow alloc];
        let rect_clone = rect.clone();
        msg![instance init initWithContentRect:rect_clone styleMask:mask backing:backing defer:false screen:screen];

        let title = str_to_nsstring(name);

        msg![instance set_title setTitle:title];
        if centered {
            msg![instance center];
        }

        let id: isize = msg_ret![instance id];
        let id = id as usize;

        let mut this = Self {
            ptr: instance,
            id,
            rect,
            top_mouse_border: NSRect::new(0., 0., 0., 0.),
            bottom_mouse_border: NSRect::new(0., 0., 0., 0.),
            left_mouse_border: NSRect::new(0., 0., 0., 0.),
            right_mouse_border: NSRect::new(0., 0., 0., 0.),
            top_left_mouse_border: NSRect::new(0., 0., 0., 0.),
            top_right_mouse_border: NSRect::new(0., 0., 0., 0.),
            bottom_left_mouse_border: NSRect::new(0., 0., 0., 0.),
            bottom_right_mouse_border: NSRect::new(0., 0., 0., 0.),
            resize_direction: None,
            hover_border: None,
        };
        this.recalculate_window_rect();
        this.make_main(backend);

        this
    }

    /// Makes the window the frontmost window.
    pub fn make_main(&self, backend: &mut MacosBackend) {
        let instance = self.ptr;
        let sender: *mut c_void = std::ptr::null_mut();
        let (make_key_and_order_front, make_main) = VTables::with(|vtables| {
            (
                vtables.nswindow.make_key_and_order_front_sel,
                vtables.nswindow.make_main_window_sel,
            )
        });

        msg![instance make_key_and_order_front makeKeyAndOrderFront:sender];
        msg![instance make_main];
        backend.frontmost_window = Some(self.id);
    }

    /// Update location, size, and border locations
    pub fn recalculate_window_rect(&mut self) {
        let frame = VTables::with(|vtables| vtables.nswindow.frame_sel);
        let instance = self.ptr;
        self.rect = msg_ret![instance frame];
        self.recalculate_window_borders();
    }

    pub fn recalculate_window_borders(&mut self) {
        let width = self.rect.size.width;
        let height = self.rect.size.height;

        self.top_mouse_border.origin.x = Self::BORDER_SIZE;
        self.top_mouse_border.origin.y = height - (Self::BORDER_SIZE / 2.);
        self.top_mouse_border.size.width = width - (Self::BORDER_SIZE * 2.);
        self.top_mouse_border.size.height = Self::BORDER_SIZE;

        self.bottom_mouse_border.origin.x = Self::BORDER_SIZE;
        self.bottom_mouse_border.origin.y = -(Self::BORDER_SIZE / 2.);
        self.bottom_mouse_border.size.width = width - (Self::BORDER_SIZE * 2.);
        self.bottom_mouse_border.size.height = Self::BORDER_SIZE;

        self.left_mouse_border.origin.x = -(Self::BORDER_SIZE / 2.);
        self.left_mouse_border.origin.y = Self::BORDER_SIZE;
        self.left_mouse_border.size.width = Self::BORDER_SIZE;
        self.left_mouse_border.size.height = height - (Self::BORDER_SIZE * 2.);

        self.right_mouse_border.origin.x = width - (Self::BORDER_SIZE / 2.);
        self.right_mouse_border.origin.y = Self::BORDER_SIZE;
        self.right_mouse_border.size.width = Self::BORDER_SIZE;
        self.right_mouse_border.size.height = height - (Self::BORDER_SIZE * 2.);

        self.top_left_mouse_border.origin.x = -(Self::BORDER_SIZE / 2.);
        self.top_left_mouse_border.origin.y = height - (Self::BORDER_SIZE / 2.);
        self.top_left_mouse_border.size.width = Self::BORDER_SIZE;
        self.top_left_mouse_border.size.height = Self::BORDER_SIZE;

        self.top_right_mouse_border.origin.x = width - (Self::BORDER_SIZE / 2.);
        self.top_right_mouse_border.origin.y = height - (Self::BORDER_SIZE / 2.);
        self.top_right_mouse_border.size.width = Self::BORDER_SIZE;
        self.top_right_mouse_border.size.height = Self::BORDER_SIZE;

        self.bottom_left_mouse_border.origin.x = -(Self::BORDER_SIZE / 2.);
        self.bottom_left_mouse_border.origin.y = -(Self::BORDER_SIZE / 2.);
        self.bottom_left_mouse_border.size.width = Self::BORDER_SIZE;
        self.bottom_left_mouse_border.size.height = Self::BORDER_SIZE;

        self.bottom_right_mouse_border.origin.x = width - (Self::BORDER_SIZE / 2.);
        self.bottom_right_mouse_border.origin.y = -(Self::BORDER_SIZE / 2.);
        self.bottom_right_mouse_border.size.width = Self::BORDER_SIZE;
        self.bottom_right_mouse_border.size.height = Self::BORDER_SIZE;
    }

    /// Check if the mouse is dragging in a certain direction.
    pub fn update_resize_direction(&mut self, mouse_pos: &NSPoint) {
        self.resize_direction = if self.top_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Top)
        } else if self.bottom_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Bottom)
        } else if self.left_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Left)
        } else if self.right_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Right)
        } else if self.top_left_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::TopLeft)
        } else if self.top_right_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::TopRight)
        } else if self.bottom_left_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::BottomLeft)
        } else if self.bottom_right_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::BottomRight)
        } else {
            None
        };
    }

    /// Check if the mouse is hovering over a window border.
    pub fn update_hover_border(&mut self, mouse_pos: &NSPoint) {
        self.hover_border = if self.top_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Top)
        } else if self.bottom_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Bottom)
        } else if self.left_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Left)
        } else if self.right_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::Right)
        } else if self.top_left_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::TopLeft)
        } else if self.top_right_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::TopRight)
        } else if self.bottom_left_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::BottomLeft)
        } else if self.bottom_right_mouse_border.contains(mouse_pos) {
            Some(NSWindowBorder::BottomRight)
        } else {
            None
        };
    }

    /// Convert a screen-space coordinate to a window-space coordinate.
    pub fn screen_point_to_local_point(&self, point: NSPoint) -> NSPoint {
        let x = point.x - self.rect.origin.x;
        let y = point.y - self.rect.origin.y;
        // TODO: invert y-coordinate for mouse events in Lokinit's API
        // let y = self.rect.size.height - y - 1.;

        NSPoint { x, y }
    }

    /// Sets the underlying NSWindow's size to our size.
    pub fn apply_size(&self) {
        let set_frame = VTables::with(|vtables| vtables.nswindow.set_frame_sel);
        let instance = self.ptr;
        let frame = self.rect.clone();

        msg![instance set_frame setFrame:frame display:false];
    }
}

// endregion: NSWindow

// region: NSEvent

#[derive(Debug)]
pub struct NSEvent {
    pub ptr: *mut c_void,
}
impl NSEvent {
    pub fn event_subtype(&self) -> NSEventSubtype {
        let event_subtype = VTables::with(|vtables| vtables.nsevent.subtype_sel);
        let instance = self.ptr;

        msg_ret![instance event_subtype]
    }

    pub fn handle(self, backend: &mut MacosBackend) -> Option<Event> {
        let (nsevent, event_type, event_subtype, window_id, mouse_pos) = VTables::with(|vtables| {
            (
                vtables.nsevent.class,
                vtables.nsevent.type_sel,
                vtables.nsevent.subtype_sel,
                vtables.nsevent.window_number_sel,
                vtables.nsevent.mouse_location_sel,
            )
        });
        let instance = self.ptr;

        let window_id: isize = msg_ret![instance window_id];
        let window_id = window_id as usize;
        let event_type: NSEventType = msg_ret![instance event_type];

        match event_type {
            // Several of these event types use private APIs, forcing us to forward the event to
            // AppKit (trying to access those APIs could result in apps using Lokinit getting
            // rejected from the app store). This, however, is pretty much the only time
            // we do so.
            NSEventType::AppKitDefined => {
                let event_subtype: NSEventSubtype = msg_ret![instance event_subtype];

                match event_subtype {
                    NSEventSubtype::ApplicationActivated
                    | NSEventSubtype::ApplicationDeactivated => {
                        NSApp::send_event(self);
                        None
                    }
                    NSEventSubtype::WindowMoved => {
                        let window = backend.windows.get_mut(&window_id).unwrap();
                        // Yes, we will get window moved events while trying to resize... pretty cringe ngl
                        if window.resize_direction.is_some() {
                            return None;
                        }

                        NSApp::send_event(self);
                        window.recalculate_window_rect();

                        Some(Event {
                            time: Duration::ZERO,
                            window: WindowHandle(window_id),
                            kind: EventKind::Moved(
                                window.rect.origin.x as i32,
                                window.rect.origin.y as i32,
                            ),
                        })
                    }
                    _ => None,
                }
            }

            NSEventType::MouseMoved
            | NSEventType::LeftMouseDragged
            | NSEventType::RightMouseDragged => {
                let window = backend
                    .windows
                    .get_mut(&backend.frontmost_window.unwrap())
                    .unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let mouse_pos = window.screen_point_to_local_point(mouse_pos);

                if let Some(border) = window.resize_direction {
                    match border {
                        NSWindowBorder::Top => {
                            window.rect.size.height = mouse_pos.y;
                        }
                        NSWindowBorder::Bottom => {
                            window.rect.origin.y += mouse_pos.y;
                            window.rect.size.height -= mouse_pos.y;
                        }
                        NSWindowBorder::Left => {
                            window.rect.origin.x += mouse_pos.x;
                            window.rect.size.width -= mouse_pos.x;
                        }
                        NSWindowBorder::Right => {
                            window.rect.size.width = mouse_pos.x;
                        }
                        NSWindowBorder::TopLeft => {
                            window.rect.size.height = mouse_pos.y;
                            window.rect.origin.x += mouse_pos.x;
                            window.rect.size.width -= mouse_pos.x;
                        }
                        NSWindowBorder::TopRight => {
                            window.rect.size.height = mouse_pos.y;
                            window.rect.size.width = mouse_pos.x;
                        }
                        NSWindowBorder::BottomLeft => {
                            window.rect.origin.y += mouse_pos.y;
                            window.rect.size.height -= mouse_pos.y;
                            window.rect.origin.x += mouse_pos.x;
                            window.rect.size.width -= mouse_pos.x;
                        }
                        NSWindowBorder::BottomRight => {
                            window.rect.origin.y += mouse_pos.y;
                            window.rect.size.height -= mouse_pos.y;
                            window.rect.size.width = mouse_pos.x;
                        }
                    }
                    window.apply_size();
                    window.recalculate_window_borders();

                    None
                } else {
                    window.update_hover_border(&mouse_pos);

                    Cursors::with(|cursors| {
                        if let Some(border) = window.hover_border {
                            match border {
                                NSWindowBorder::TopLeft | NSWindowBorder::BottomRight => {
                                    cursors.get(MacOsCursor::ResizeNorthWestSouthEast)
                                }
                                NSWindowBorder::TopRight | NSWindowBorder::BottomLeft => {
                                    cursors.get(MacOsCursor::ResizeNorthEastSouthWest)
                                }
                                NSWindowBorder::Top | NSWindowBorder::Bottom => {
                                    cursors.get(MacOsCursor::ResizeNorthSouth)
                                }
                                NSWindowBorder::Left | NSWindowBorder::Right => {
                                    cursors.get(MacOsCursor::ResizeEastWest)
                                }
                            }
                            .set();

                            None
                        } else {
                            cursors.get(MacOsCursor::Arrow).set();

                            Some(Event {
                                time: Duration::ZERO,
                                window: WindowHandle(window_id),
                                kind: EventKind::Mouse(MouseEvent::CursorMove(
                                    mouse_pos.x as i32,
                                    mouse_pos.y as i32,
                                )),
                            })
                        }
                    })
                }
            }

            NSEventType::LeftMouseDown => {
                let window = backend.windows.get_mut(&window_id).unwrap();

                if window.hover_border.is_some() {
                    window.resize_direction = window.hover_border;
                    None
                } else {
                    let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                    let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                    Some(Event {
                        time: Duration::ZERO,
                        window: WindowHandle(window_id),
                        kind: EventKind::Mouse(MouseEvent::ButtonPress(
                            MouseButton::Left,
                            x as i32,
                            y as i32,
                        )),
                    })
                }
            }
            NSEventType::LeftMouseUp => {
                let window = backend.windows.get_mut(&window_id).unwrap();

                if window.resize_direction.is_some() {
                    window.resize_direction = None;
                    Cursors::with(|cursors| cursors.get(MacOsCursor::Arrow).set());
                    println!("Bruh");
                    None
                } else {
                    let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                    let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                    Some(Event {
                        time: Duration::ZERO,
                        window: WindowHandle(window_id),
                        kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                            MouseButton::Left,
                            x as i32,
                            y as i32,
                        )),
                    })
                }
            }

            NSEventType::RightMouseDown => {
                let window = backend.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Mouse(MouseEvent::ButtonPress(
                        MouseButton::Right,
                        x as i32,
                        y as i32,
                    )),
                })
            }
            NSEventType::RightMouseUp => {
                let window = backend.windows.get(&window_id).unwrap();
                let mouse_pos: NSPoint = msg_ret![nsevent mouse_pos];
                let NSPoint { x, y } = window.screen_point_to_local_point(mouse_pos);

                Some(Event {
                    time: Duration::ZERO,
                    window: WindowHandle(window_id),
                    kind: EventKind::Mouse(MouseEvent::ButtonRelease(
                        MouseButton::Right,
                        x as i32,
                        y as i32,
                    )),
                })
            }

            _ => None,
        }
    }
}

// endregion: NSEvent

// region: Utils

pub fn str_to_nsstring(string: &str) -> *mut c_void {
    let (nsstring, alloc, init) = VTables::with(|vtables| {
        (
            vtables.nsstring.class,
            vtables.nsstring.alloc,
            vtables.nsstring.init_with_bytes_length_encoding_sel,
        )
    });
    let nsstring = msg_ret![nsstring alloc];
    let bytes = string.as_ptr();
    let length = string.len();
    msg_ret![nsstring init initWithBytes:bytes length:length encoding:4usize]
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}
impl NSRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            origin: NSPoint { x, y },
            size: NSSize { width, height },
        }
    }

    pub fn contains(&self, point: &NSPoint) -> bool {
        point.x > self.origin.x
            && point.y > self.origin.y
            && point.x < (self.origin.x + self.size.width)
            && point.y < (self.origin.y + self.size.height)
    }
}

// endregion: Utils
