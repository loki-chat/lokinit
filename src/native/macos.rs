mod event_handler;
mod keysym;
#[cfg(feature = "opengl")]
pub mod opengl;
mod window;

#[cfg(feature = "opengl")]
use {
    crate::gl::*,
    loki_mac::dynload::{load_opengl, Library},
    std::ffi::{c_char, c_void},
};
use {
    crate::{
        event::{Event, EventKind},
        keycode::KeyCode,
        lok::{CreateWindowError, LokinitBackend},
        prelude::Monitor,
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
    pub windows: HashMap<isize, Window>,
    /// The ID of the window that's currently in front.
    pub frontmost_window: Option<isize>,
    /// Queued events that haven't been handled yet. This is used for anything that triggers
    /// 2 events. For example, switching windows triggers both a `FocusIn` event for the newly main
    /// window and a `FocusOut` event for the formerly main window.
    pub event_queue: VecDeque<Event>,
    /// If a window is resizing, the direction it's resizing in.
    pub resize_direction: Option<WindowBorder>,
    /// Currently pressed modifier keys. Modifier keys are shift, command, alt, and control.
    pub active_modifiers: HashSet<KeyCode>,
    /// An underlying AppKit `NSApplication` instance.
    pub nsapp: NSApplication,
    /// The `OpenGL.framework` library, for loading OpenGL functions.
    #[cfg(feature = "opengl")]
    pub opengl: Library,
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
        self.windows.get_mut(&new_window_id).unwrap().focus();
    }
}

impl LokinitBackend for MacosBackend {
    fn init() -> Self {
        let mut nsapp = NSApplication::shared();
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
            #[cfg(feature = "opengl")]
            opengl: load_opengl(),
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
        let window_id = window.id();

        if builder.centered {
            window.center();
        }
        window.set_title(NSString::from_str(builder.title));
        window.focus();

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

    fn set_screen_mode(&mut self, _handle: WindowHandle, _screen_mode: crate::window::ScreenMode) {
        todo!()
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            if let Some(event) = self.event_queue.pop_front() {
                return Some(event);
            }

            let raw_event = self
                .nsapp
                .next_event(
                    NSEventMask::Any,
                    NSDate::distant_future(),
                    NSRunLoopMode::default(),
                    true.into(),
                )
                .unwrap();
            self.handle_raw_event(raw_event);
        }
    }

    fn fetch_monitors(&mut self) -> Vec<Monitor> {
        todo!()
    }

    #[cfg(feature = "opengl")]
    fn create_window_surface(
        &mut self,
        window_handle: WindowHandle,
        _cfg: OpenGLConfig,
    ) -> WindowSurface {
        let window = self.windows.get_mut(&window_handle.0).unwrap();
        let view = window.content_view();

        let mut attrs = Vec::new();
        attrs.push(NSOpenGLPFA::Accelerated as _);
        attrs.push(NSOpenGLPFA::DoubleBuffer as _);
        attrs.push(NSOpenGLPFA::OpenGLProfile as _);
        attrs.push(NSOpenGLProfile::Core4_1 as _);
        // Attributes must end in 0
        attrs.push(0);
        let pixel_format = NSOpenGLPixelFormat::new(&attrs);

        let mut context = NSOpenGLContext::new(pixel_format);
        context.set_view(view);

        // HACK: For some reason the context is the wrong size before a window resize.
        // Adding a fake resize event here forces it to become the right size.
        // I'm not sure what about the resize causes it to be fixed. In the resize handler
        // we call `context.update()`, but calling that here too doesn't fix anything.
        let size = window.frame().size;
        self.event_queue.push_back(Event {
            time: Duration::ZERO,
            window: window_handle,
            kind: EventKind::Resized(size.width as _, size.height as _),
        });

        window.gl_context = Some(context);

        WindowSurface {
            window: window_handle,
        }
    }
    #[cfg(feature = "opengl")]
    fn load_opengl_func(&mut self, proc_name: *const c_char) -> Option<*mut c_void> {
        self.opengl.load(proc_name)
    }
    #[cfg(feature = "opengl")]
    fn make_surface_active(&self, surface: WindowSurface) {
        self.windows
            .get(&surface.window.0)
            .unwrap()
            .gl_context
            .as_ref()
            .unwrap()
            .make_current();
    }
    #[cfg(feature = "opengl")]
    fn flush_surface(&self, surface: WindowSurface) {
        self.windows
            .get(&surface.window.0)
            .unwrap()
            .gl_context
            .as_ref()
            .unwrap()
            .flush_buffer();
    }
    #[cfg(feature = "opengl")]
    fn update_surface(&self, surface: WindowSurface) {
        self.windows
            .get(&surface.window.0)
            .unwrap()
            .gl_context
            .as_ref()
            .unwrap()
            .update();
    }
}
