mod bplist;
mod keysym;
mod objc;

use crate::event::EventKind;
use std::time::Duration;
use {
    crate::{
        event::Event,
        lok::{CreateWindowError, LokinitBackend},
        window::{WindowBuilder, WindowHandle},
    },
    objc::{cursor, macros::*, vtables::VTables, NSApp, NSRect, NSWindow},
    std::{collections::HashMap, collections::VecDeque, ffi::c_void},
};

pub struct MacosBackend {
    /// Maps to Loki's windows by the window's ID.
    pub windows: HashMap<usize, NSWindow>,
    /// The ID of the window that's currently in front.
    pub frontmost_window: Option<usize>,
    /// Queued events that haven't been handled yet. This is used for anything that triggers
    /// 2 events. For example, switching windows triggers both a `FocusIn` event for the newly main
    /// window and a `FocusOut` event for the formerly main window.
    pub event_queue: VecDeque<Event>,
}

impl MacosBackend {
    /// Convenience method to get the NSWindow instance for the main window. The backend only
    /// stores the ID of the main window, but this method will find the actual object for that
    /// window using its ID.
    pub fn get_frontmost_window(&mut self) -> Option<&mut NSWindow> {
        self.windows.get_mut(&self.frontmost_window?)
    }

    /// Changes the frontmost window ID in Lokinit, and sends messages to the NSWindow to make it
    /// appear on top of other windows. Also queues FocusIn/FocusOut events as needed.
    pub fn set_frontmost_window(&mut self, new_window_id: usize) {
        if let Some(old_window_id) = self.frontmost_window {
            if old_window_id == new_window_id {
                return;
            }

            self.event_queue.push_back(Event {
                // TODO: Time
                time: Duration::ZERO,
                window: WindowHandle(old_window_id),
                kind: EventKind::FocusOut,
            });
        }

        self.event_queue.push_back(Event {
            // TODO: Time
            time: Duration::ZERO,
            window: WindowHandle(new_window_id),
            kind: EventKind::FocusIn,
        });

        self.frontmost_window = Some(new_window_id);

        // We have to call 2 separate methods to make windows the main window.
        // The first makes the window the "key" window, so it can receive keystrokes, and moves
        // it to the front. The second makes the window focused/active.
        let new_window = self.windows.get_mut(&new_window_id).unwrap();
        let instance = new_window.ptr;
        let sender: *mut c_void = std::ptr::null_mut();
        let (make_key_and_order_front, make_main) = VTables::with(|vtables| {
            (
                vtables.nswindow.make_key_and_order_front_sel,
                vtables.nswindow.make_main_window_sel,
            )
        });
        msg![instance make_key_and_order_front makeKeyAndOrderFront:sender];
        msg![instance make_main];
    }
}

impl LokinitBackend for MacosBackend {
    fn init() -> Self {
        VTables::init();
        cursor::load_cursors();
        NSApp.load();

        Self {
            windows: HashMap::new(),
            frontmost_window: None,
            event_queue: VecDeque::new(),
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
        todo!("Window closing support")
    }

    fn poll_event(&mut self) -> Option<Event> {
        // Prioritize queued events; if there aren't any, process the next NSEvent.
        if let Some(event) = self.event_queue.pop_front() {
            return Some(event);
        }

        loop {
            let event = NSApp.next_event();

            if let Some(event) = NSApp.handle(event, self) {
                return Some(event);
            }
        }
    }
}
