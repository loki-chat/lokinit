mod bplist;
mod keysym;
mod objc;

use {
    crate::{
        event::{Event, EventKind},
        keycode::KeyCode,
        lok::{CreateWindowError, LokinitBackend},
        window::{WindowBuilder, WindowHandle},
    },
    objc::{
        cursor, cursor::MacOsCursor, enums::NSApplicationActivationPolicy, macros::*,
        vtables::VTables, NSEvent, NSRect, NSWindow,
    },
    std::{collections::HashMap, collections::VecDeque, ffi::c_void, time::Duration},
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
    /// Selector and class pointers for Objective-C bindings.
    pub vtables: VTables,
    /// Pointers to the macOS cursor classes.
    pub cursors: HashMap<MacOsCursor, *mut c_void>,
    /// If a window is resizing.
    pub in_resize: bool,
    /// The previously pressed key - used for `RepeatKey` events.
    pub last_key: Option<KeyCode>,
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
        let (make_key_and_order_front, make_main) = (
            self.vtables.nswindow.make_key_and_order_front_sel,
            self.vtables.nswindow.make_main_window_sel,
        );
        msg![instance make_key_and_order_front makeKeyAndOrderFront:sender];
        msg![instance make_main];
    }

    /// Waits until the next NSEvent is sent to the app, then returns it.
    pub fn wait_on_next_nsevent(&self) -> NSEvent {
        let (nsapp, next_event, distant_future) = (
            self.vtables.nsapp.shared,
            self.vtables.nsapp.next_event_matching_sel,
            self.vtables.nsdate.distant_future,
        );
        // Matches all NSEvent masks
        let mask = usize::MAX;
        let mode = unsafe { objc::ffi::NSDefaultRunLoopMode };

        let ptr = msg_ret![nsapp next_event nextEventMatchingMask:mask untilDate:distant_future inMode:mode dequeue:true];
        NSEvent { ptr }
    }

    /// Change the active cursor.
    pub fn set_cursor(&self, cursor: MacOsCursor) {
        if self.in_resize {
            return;
        }

        let set = self.vtables.nscursor.set_sel;
        let ptr = self.cursors.get(&cursor).unwrap().to_owned();
        msg![ptr set];
    }
}

impl LokinitBackend for MacosBackend {
    fn init() -> Self {
        let vtables = VTables::default();
        let cursors = cursor::load_cursors(&vtables);

        let (instance, set_activation_policy, activate, finish_launching) = (
            vtables.nsapp.shared,
            vtables.nsapp.set_activation_policy_sel,
            vtables.nsapp.activate_ignoring_other_apps_sel,
            vtables.nsapp.finish_launching_sel,
        );

        let activation_policy = NSApplicationActivationPolicy::Regular as usize;

        msg![instance set_activation_policy setActivationPolicy:activation_policy];
        msg![instance activate activateIgnoringOtherApps:true];
        msg![instance finish_launching];

        Self {
            windows: HashMap::new(),
            frontmost_window: None,
            event_queue: VecDeque::new(),
            vtables,
            cursors,
            in_resize: false,
            last_key: None,
        }
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        let rect = NSRect::new(
            builder.position.x as f64,
            builder.position.y as f64,
            builder.size.width as f64,
            builder.size.height as f64,
        );
        let id = NSWindow::new_in_backend(rect, builder.centered, &builder.title, self);

        Ok(WindowHandle(id))
    }

    fn close_window(&mut self, handle: WindowHandle) {
        let window_id = handle.0;
        let Some(window) = self.windows.get(&window_id) else {
            return;
        };

        let close = self.vtables.nswindow.close_sel;
        let ptr = window.ptr;
        msg![ptr close];
        self.windows.remove(&window_id);

        if self.frontmost_window == Some(window_id) {
            // TODO: Should we make a different window main?
            self.frontmost_window = None;
        }
    }

    fn set_screen_mode(&mut self, handle: WindowHandle, screen_mode: crate::window::ScreenMode) {
        todo!()
    }

    fn poll_event(&mut self) -> Option<Event> {
        // Prioritize queued events; if there aren't any, process the next NSEvent.
        if let Some(event) = self.event_queue.pop_front() {
            return Some(event);
        }

        loop {
            let event = self.wait_on_next_nsevent();

            if let Some(event) = self.handle(event) {
                return Some(event);
            }
        }
    }
}
