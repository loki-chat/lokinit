//! FFI functions for Swift to use

use crate::event::{MouseButton, MouseEvent};

use {
    super::{
        ffi_swift::{SwiftMouseButton, SwiftMouseEvent},
        EVENT_QUEUE,
    },
    crate::{
        core,
        event::{Event, EventKind},
        window::WindowHandle,
    },
    std::time::Duration,
};

/// Lokinit's mouse event callback
#[no_mangle]
pub extern "C" fn rust_mouse_callback(
    window: i32,
    mouse_btn: SwiftMouseButton,
    mouse_event: SwiftMouseEvent,
    x: f64,
    y: f64,
) {
    EVENT_QUEUE.with(move |queue| {
        let mouse_event = mouse_event.into_mouse_event(x, y, mouse_btn);

        queue.borrow_mut().push_back(Event {
            time: Duration::ZERO,
            window: WindowHandle(window as usize),
            kind: EventKind::Mouse(mouse_event),
        });
    });
}

/// Lokinit's window resize callback
#[no_mangle]
pub extern "C" fn rust_window_resize_callback(window: usize, width: u32, height: u32) {
    EVENT_QUEUE.with(move |queue| {
        queue.borrow_mut().push_back(Event {
            time: Duration::ZERO,
            window: WindowHandle(window),
            kind: EventKind::Resized(width, height),
        });
    });
}
