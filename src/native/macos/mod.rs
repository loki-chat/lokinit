/// FFI that's in Swift code
mod swift {
    use {
        crate::event::{MouseButton, MouseEvent},
        std::ffi::{c_char, c_void},
    };

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

    #[repr(i32)]
    pub enum SwiftMouseEvent {
        Pressed = 0,
        Released = 1,
        Moved = 2,
    }
    impl SwiftMouseEvent {
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
        pub fn setup();
        pub fn create_window(width: i64, height: i64, title: *const c_char) -> u64;
        pub fn next_event() -> bool;
    }
}

/// FFI that's in Rust code
pub mod rust {
    use crate::event::{MouseButton, MouseEvent};

    use {
        super::{
            swift::{SwiftMouseButton, SwiftMouseEvent},
            EVENT_QUEUE,
        },
        crate::{
            core,
            event::{Event, EventKind},
            window::WindowHandle,
        },
        std::time::Duration,
    };

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
}

#[derive(Debug)]
pub enum NativeCoreError {}
#[derive(Debug)]
pub enum CreateWindowError {}

use {
    crate::{
        core::Monitor,
        event::Event,
        window::{WindowBuilder, WindowHandle},
    },
    std::{cell::RefCell, collections::VecDeque, ffi::CString},
};

thread_local! {
    static EVENT_QUEUE: RefCell<VecDeque<Event>> = RefCell::new(VecDeque::new());
}

pub struct LokinitCore {
    terminated: bool,
}

impl LokinitCore {
    pub fn init() -> Self {
        unsafe { swift::setup() };
        Self { terminated: true }
    }

    pub fn fetch_monitors() -> Vec<Monitor> {
        todo!()
    }

    pub fn create_window(
        &mut self,
        builder: WindowBuilder,
    ) -> Result<WindowHandle, CreateWindowError> {
        let title = CString::new(builder.title).expect("Invalid window title");
        let window_id = unsafe {
            swift::create_window(
                builder.size.width as i64,
                builder.size.height as i64,
                title.as_ptr(),
            )
        };

        Ok(WindowHandle(window_id as usize))
    }

    pub fn poll_event(&self) -> Option<Event> {
        let mut event = None;
        while event.is_none() {
            // next_event will return `True` if the app should terminate
            if unsafe { swift::next_event() } {
                return None;
            }
            event = EVENT_QUEUE.with(|queue| queue.borrow_mut().pop_front());
        }
        event
    }
}
