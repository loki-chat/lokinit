use crate::lok::{CreateWindowError, LokinitBackend};

mod bplist;
mod keysym;
mod objc;

use {
    crate::{
        event::Event,
        window::{WindowBuilder, WindowHandle},
    },
    objc::{cursor, vtables, NSApp, NSRect, NSWindow},
    std::{cell::RefCell, collections::HashMap, collections::VecDeque, ffi::CString},
};

pub struct MacosBackend {
    /// Maps to Loki's windows by the window's ID.
    windows: HashMap<usize, NSWindow>,
    /// The ID of the window that's currently in front.
    frontmost_window: Option<usize>,
}

impl MacosBackend {
    pub fn get_frontmost_window(&mut self) -> Option<&mut NSWindow> {
        self.windows.get_mut(&self.frontmost_window?)
    }
}

impl LokinitBackend for MacosBackend {
    fn init() -> Self {
        vtables::init_vtables();
        cursor::load_cursors();
        NSApp::load();

        Self {
            windows: HashMap::new(),
            frontmost_window: None,
        }
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        let rect = NSRect::new(
            builder.position.x as f64,
            builder.position.y as f64,
            builder.size.width as f64,
            builder.size.height as f64,
        );
        let window = NSWindow::new(rect, builder.centered, &builder.title, self);
        let id = window.id;
        self.windows.insert(id, window);

        Ok(WindowHandle(id))
    }

    fn close_window(&mut self, handle: WindowHandle) {
        todo!()
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            let event = NSApp::next_event();

            if let Some(event) = event.handle(self) {
                return Some(event);
            }
        }

        None
    }
}
