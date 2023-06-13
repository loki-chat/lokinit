mod ffi_rust;
mod ffi_swift;

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

pub struct LokinitCore;

impl LokinitCore {
    pub fn init() -> Result<Self, NativeCoreError> {
        unsafe { ffi_swift::setup() };
        Ok(Self)
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
            ffi_swift::create_window(
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
            // update() will return `True` if the app should terminate
            if unsafe { ffi_swift::update() } {
                return None;
            }
            event = EVENT_QUEUE.with(|queue| queue.borrow_mut().pop_front());
        }
        event
    }
}
