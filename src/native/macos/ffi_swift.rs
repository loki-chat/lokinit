//! Bindings to the external Swift code

use {
    crate::event::{MouseButton, MouseEvent},
    std::ffi::c_char,
};

/// The MouseButton enum, exported for Swift
#[repr(i32)]
pub enum SwiftMouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
}
impl From<SwiftMouseButton> for MouseButton {
    fn from(value: SwiftMouseButton) -> Self {
        match value {
            SwiftMouseButton::Left => MouseButton::Left,
            SwiftMouseButton::Right => MouseButton::Right,
            SwiftMouseButton::Middle => MouseButton::Middle,
        }
    }
}

/// The MouseEvent enum, exported for Swift
#[repr(i32)]
pub enum SwiftMouseEvent {
    Pressed = 0,
    Released = 1,
    Moved = 2,
}

impl SwiftMouseEvent {
    /// Translates the SwiftMouseEvent enum into Lokinit's MouseEvent enum
    pub fn into_mouse_event(self, x: f64, y: f64, button: SwiftMouseButton) -> MouseEvent {
        let x = x as i32;
        let y = y as i32;
        let button = button.into();

        match self {
            Self::Pressed => MouseEvent::ButtonPress(button, x, y),
            Self::Released => MouseEvent::ButtonRelease(button, x, y),
            Self::Moved => MouseEvent::CursorMove(x, y),
        }
    }
}

extern "C" {
    /// Initializes the NSApplication
    pub fn setup();

    /// Creates a new window
    pub fn create_window(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        centered: bool,
        title: *const c_char,
    ) -> u64;

    /// Updates the app state by processing queued events in the NSApplication.
    ///
    /// Without manually calling this to update events, we'd have to use
    /// NSApplication.run(), which takes control of the thread and starts
    /// the event loop. Instead, Lokinit calls this each time `poll_event()`
    /// is called, which updates the app state without getting stuck in Apple's
    /// run loop.
    pub fn update() -> bool;
}
