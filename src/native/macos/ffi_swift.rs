//! Bindings to the external Swift code

use {
    crate::{
        event::{Event, EventKind, MouseButton, MouseEvent},
        window::WindowHandle,
    },
    std::{ffi::c_char, time::Duration},
};

#[repr(i32)]
#[allow(dead_code)]
pub enum SwiftEventType {
    MouseDownLeft,
    MouseDownMiddle,
    MouseDownRight,
    MouseDownOther,

    MouseUpLeft,
    MouseUpMiddle,
    MouseUpRight,
    MouseUpOther,

    MouseMoved,
    MouseEntered,
    MouseExited,
    MouseScrolled,

    WindowResized,
    WindowMoved,
    WindowCloseRequested,
    WindowDestroyed,
    WindowGainedFocus,
    WindowLostFocus,

    KeyPressed,
    KeyReleased,
    KeyRepeated,

    AppQuit,
}
#[repr(C)]
pub struct SwiftEvent {
    pub kind: SwiftEventType,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub window: usize,
}
impl TryInto<Event> for SwiftEvent {
    type Error = ();

    fn try_into(self) -> Result<Event, Self::Error> {
        let kind = match self.kind {
            SwiftEventType::MouseDownLeft => EventKind::Mouse(MouseEvent::ButtonPress(
                MouseButton::Left,
                self.data1,
                self.data2,
            )),
            SwiftEventType::MouseUpLeft => EventKind::Mouse(MouseEvent::ButtonRelease(
                MouseButton::Left,
                self.data1,
                self.data2,
            )),
            SwiftEventType::MouseDownRight => EventKind::Mouse(MouseEvent::ButtonPress(
                MouseButton::Right,
                self.data1,
                self.data2,
            )),
            SwiftEventType::MouseUpRight => EventKind::Mouse(MouseEvent::ButtonRelease(
                MouseButton::Right,
                self.data1,
                self.data2,
            )),
            SwiftEventType::MouseDownOther => EventKind::Mouse(MouseEvent::ButtonPress(
                MouseButton::Other(self.data3.try_into().unwrap()),
                self.data1,
                self.data2,
            )),
            SwiftEventType::MouseUpOther => EventKind::Mouse(MouseEvent::ButtonRelease(
                MouseButton::Other(self.data3.try_into().unwrap()),
                self.data1,
                self.data2,
            )),
            SwiftEventType::MouseMoved => {
                EventKind::Mouse(MouseEvent::CursorMove(self.data1, self.data2))
            }
            SwiftEventType::WindowResized => {
                EventKind::Resized(self.data1 as u32, self.data2 as u32)
            }
            SwiftEventType::WindowMoved => EventKind::Moved(self.data1, self.data2),
            SwiftEventType::WindowDestroyed => EventKind::Destroyed,
            SwiftEventType::MouseEntered => {
                EventKind::Mouse(MouseEvent::CursorIn(self.data1, self.data2))
            }
            SwiftEventType::MouseExited => {
                EventKind::Mouse(MouseEvent::CursorOut(self.data1, self.data2))
            }
            SwiftEventType::WindowGainedFocus => EventKind::FocusIn,
            SwiftEventType::WindowLostFocus => EventKind::FocusOut,
            _ => return Err(()),
        };

        Ok(Event {
            time: Duration::ZERO,
            window: WindowHandle(self.window),
            kind,
        })
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
    pub fn update() -> SwiftEvent;
}
