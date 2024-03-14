mod event_handler;
mod keysym;
mod window;

use {
    crate::{
        event::{Event, EventKind},
        keycode::KeyCode,
        lok::{CreateWindowError, LokinitBackend},
        window::{WindowBorder, WindowBuilder, WindowHandle},
    },
    loki_mac::*,
    std::{
        collections::{HashMap, HashSet, VecDeque},
        time::Duration,
    },
    window::Window,
};

pub struct MacosBackend {
    /// Maps to Loki's windows by the window's ID.
    pub windows: HashMap<usize, Window>,
    /// The ID of the window that's currently in front.
    pub frontmost_window: Option<usize>,
    /// Queued events that haven't been handled yet. This is used for anything that triggers
    /// 2 events. For example, switching windows triggers both a `FocusIn` event for the newly main
    /// window and a `FocusOut` event for the formerly main window.
    pub event_queue: VecDeque<Event>,
    /// If a window is resizing, the direction it's resizing in.
    pub resize_direction: Option<WindowBorder>,
    /// Currently pressed modifier keys. Modifier keys are shift, command, alt, and control.
    pub active_modifiers: HashSet<KeyCode>,
    /// An underlying AppKit `NSApplication` instance.
    pub nsapp: NSApp,
}

impl MacosBackend {
    /// Changes the frontmost window ID in Lokinit, and sends messages to the NSWindow to make it
    /// appear on top of other windows. Also queues FocusIn/FocusOut events as needed.
    pub fn set_frontmost_window(&mut self, new_window: WindowHandle) {
        let new_window_id = new_window.0;
        if self.frontmost_window == Some(new_window_id) {
            return;
        }

        if let Some(old_window_id) = self.frontmost_window {
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
        self.windows.get_mut(&new_window_id).unwrap().make_main();
    }
}

impl LokinitBackend for MacosBackend {
    fn init() -> Self {
        let mut nsapp = NSApp::shared();
        nsapp.activate();
        nsapp.finish_launching();
        nsapp.set_activation_policy(NSApplicationActivationPolicy::Regular);

        Self {
            windows: HashMap::new(),
            frontmost_window: None,
            event_queue: VecDeque::new(),
            resize_direction: None,
            active_modifiers: HashSet::default(),
            nsapp,
        }
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        let size = NSRect {
            size: NSSize {
                width: builder.size.width as f64,
                height: builder.size.height as f64,
            },
            origin: NSPoint {
                x: builder.position.x as f64,
                y: builder.position.y as f64,
            },
        };

        let mut window = NSWindow::new(
            size,
            NSWindowStyleMask::default()
                .closable()
                .miniaturizable()
                .resizable()
                .titled(),
        );
        let window_id: usize = window
            .id()
            .try_into()
            .expect("Lokinit error: macOS set an invalid window ID.");

        if builder.centered {
            window.center();
        }
        window.set_title(&builder.title);
        window.make_main();

        self.frontmost_window = Some(window_id);
        let old_window = self.windows.insert(window_id, Window::new(window));
        if old_window.is_some() {
            panic!("Lokinit error: macOS returned two windows with the same ID.");
        }

        Ok(WindowHandle(window_id))
    }

    fn close_window(&mut self, handle: WindowHandle) {
        let window_id = handle.0;
        let window = self.windows.remove(&window_id).unwrap();
        window.close();

        if self.frontmost_window == Some(window_id) {
            self.frontmost_window = None;
        }
    }

    fn set_screen_mode(&mut self, handle: WindowHandle, screen_mode: crate::window::ScreenMode) {
        todo!()
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            if let Some(event) = self.event_queue.pop_front() {
                return Some(event);
            }

            let raw_event = self.nsapp.next_event(
                NSEventMask::Any,
                NSDate::distant_future(),
                NSRunLoopMode::Default,
                true,
            );
            self.handle_raw_event(raw_event);
        }
    }
}
