use crate::lok::{CreateWindowError, LokinitBackend};

mod ffi_rust;
mod ffi_swift;
mod keysym;

use {
    crate::{
        event::Event,
        window::{WindowBuilder, WindowHandle},
    },
    std::{cell::RefCell, collections::VecDeque, ffi::CString},
};

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

    #[cfg(feature = "raw-window-handle")]
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        todo!()
    }

    #[cfg(feature = "raw-window-handle")]
    fn raw_window_handle_for(&self, window_handle: WindowHandle) -> raw_window_handle::RawWindowHandle {
        todo!()
    }
}
