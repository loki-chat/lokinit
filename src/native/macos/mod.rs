use crate::lok::{CreateWindowError, LokinitBackend};

mod ffi_rust;
mod ffi_swift;

use {
    crate::{
        event::Event,
        window::{WindowBuilder, WindowHandle},
    },
    std::{cell::RefCell, collections::VecDeque, ffi::CString},
};

thread_local! {
    static EVENT_QUEUE: RefCell<VecDeque<Event>> = RefCell::new(VecDeque::new());
}

pub struct MacosBackend;

impl LokinitBackend for MacosBackend {
    fn init() -> Self {
        unsafe { ffi_swift::setup() };
        Self
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        let title = CString::new(builder.title)
            .map_err(|e| CreateWindowError(format!("Invalid window title: {}", e).into()))?;

        let window_id = unsafe {
            ffi_swift::create_window(
                builder.position.x,
                builder.position.y,
                builder.size.width as i32,
                builder.size.height as i32,
                builder.centered,
                title.as_ptr(),
            )
        };

        Ok(WindowHandle(window_id as usize))
    }

    fn close_window(&mut self, handle: WindowHandle) {
        todo!()
    }

    fn poll_event(&mut self) -> Option<Event> {
        unsafe { ffi_swift::update() }.try_into().ok()
    }
}
