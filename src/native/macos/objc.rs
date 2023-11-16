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
pub mod event;
pub mod ffi;
pub mod macros;
pub mod vtables;

use {
    crate::event::*, cursor::MacOsCursor, enums::*, macros::*, std::ffi::c_void, vtables::VTables,
};

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
    /// A pointer to the underlying (Objective-C) NSWindow instance.
    pub ptr: *mut c_void,
    /// The top border of the window. If the mouse drags inside this, we should resize.
    pub top_mouse_border: NSRect,
    /// The bottom border of the window. If the mouse drags inside this, we should resize.
    pub bottom_mouse_border: NSRect,
    /// The left border of the window. If the mouse drags inside this, we should resize.
    pub left_mouse_border: NSRect,
    /// The right border of the window. If the mouse drags inside this, we should resize.
    pub right_mouse_border: NSRect,
    /// The top-left corner of the window. If the mouse drags inside this, we should resize.
    pub top_left_mouse_border: NSRect,
    /// The top-right corner of the window. If the mouse drags inside this, we should resize.
    pub top_right_mouse_border: NSRect,
    /// The bottom-left corner of the window. If the mouse drags inside this, we should resize.
    pub bottom_left_mouse_border: NSRect,
    /// The bottom-right corner of the window. If the mouse drags inside this, we should resize.
    pub bottom_right_mouse_border: NSRect,
    /// A rectangle around the stoplight buttons in the window.
    pub stoplight_buttons_rect: NSRect,
    /// The entire window's rectangle.
    pub rect: NSRect,
    /// The direction the window is currently being resized.
    pub resize_direction: Option<NSWindowBorder>,
    /// The window border the mouse is currently hovered over.
    pub hover_border: Option<NSWindowBorder>,
    /// The window's ID.
    pub id: usize,
}
impl NSWindow {
    const BORDER_SIZE: f64 = 7.;

    /// Creates a new window & stores it in the backend.
    pub fn new_in_backend(
        rect: NSRect,
        centered: bool,
        name: &str,
        backend: &mut MacosBackend,
    ) -> usize {
        let (nswindow, alloc, init, id, set_title, center, nsscreen, main_screen) = (
            backend.vtables.nswindow.class,
            backend.vtables.nswindow.alloc_sel,
            backend.vtables.nswindow.constructor_sel,
            backend.vtables.nswindow.window_number_sel,
            backend.vtables.nswindow.set_title_sel,
            backend.vtables.nswindow.center_sel,
            backend.vtables.nsscreen.class,
            backend.vtables.nsscreen.main_screen_sel,
        );

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

        let title = str_to_nsstring(name, &backend.vtables);
        msg![instance set_title setTitle:title];

        if centered {
            msg![instance center];
        }

        let id: isize = msg_ret![instance id];
        let id = id as usize;

        // Create a rectangle around the 3 stoplight buttons, so we can check if the mouse
        // is hovering over them.

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
            stoplight_buttons_rect: NSRect::new(0., 0., 0., 0.),
            resize_direction: None,
            hover_border: None,
        };

        this.recalculate_window_rect(&backend.vtables);
        backend.windows.insert(id, this);
        backend.set_frontmost_window(id);

        id
    }

    /// Update location, size, and border locations
    pub fn recalculate_window_rect(&mut self, vtables: &VTables) {
        let frame = vtables.nswindow.frame_sel;
        let instance = self.ptr;
        self.rect = msg_ret![instance frame];
        self.recalculate_window_borders(vtables);
    }

    pub fn recalculate_window_borders(&mut self, vtables: &VTables) {
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

        let (std_window_btn, superview, frame) = (
            vtables.nswindow.std_window_btn_sel,
            vtables.nsbutton.superview_sel,
            vtables.nswindow.frame_sel,
        );

        let instance = self.ptr;
        let close_btn = NSWindowButton::Close;
        let close_btn = msg_ret![instance std_window_btn standardWindowButton:close_btn];
        // let close_btn = msg_ret![close_btn superview];
        let zoom_btn = NSWindowButton::Close;
        let zoom_btn = msg_ret![instance std_window_btn standardWindowButton:zoom_btn];
        // let zoom_btn = msg_ret![zoom_btn superview];
        let titlebar = msg_ret![close_btn superview];
        let titlebar = msg_ret![titlebar superview];

        let close_btn: NSRect = msg_ret![close_btn frame];
        let zoom_btn: NSRect = msg_ret![zoom_btn frame];
        let titlebar: NSRect = msg_ret![titlebar frame];

        self.stoplight_buttons_rect = NSRect::new(
            close_btn.origin.x + titlebar.origin.x,
            close_btn.origin.y + titlebar.origin.y,
            ((zoom_btn.origin.x + zoom_btn.size.width) * 3.) - close_btn.origin.x,
            zoom_btn.size.height,
        );
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
    pub fn apply_size(&self, vtables: &VTables) {
        let set_frame = vtables.nswindow.set_frame_sel;
        let instance = self.ptr;
        let frame = self.rect.clone();

        msg![instance set_frame setFrame:frame display:false];
    }
}

#[derive(Debug)]
pub struct NSEvent {
    pub ptr: *mut c_void,
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

/// Allocates an new NSString, then sets its contents to the `&str`'s contents. Both types are UTF-8,
/// so no conversion is necessary; we can just use the `&str` as-is.
/// Returns a pointer to the NSString.
pub fn str_to_nsstring(string: &str, vtables: &VTables) -> *mut c_void {
    let (nsstring, alloc, init) = (
        vtables.nsstring.class,
        vtables.nsstring.alloc,
        vtables.nsstring.init_with_bytes_length_encoding_sel,
    );

    let nsstring = msg_ret![nsstring alloc];
    let bytes = string.as_ptr();
    let length = string.len();
    let encoding = NSStringEncoding::UTF8;

    msg_ret![nsstring init initWithBytes:bytes length:length encoding:encoding]
}
