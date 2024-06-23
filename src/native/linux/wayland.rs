use {
    crate::{
        event::Event,
        lok::{CreateWindowError, LokinitBackend},
        native::WindowId,
        prelude::{Monitor, WindowBuilder, WindowHandle},
        window::ScreenMode,
    },
    loki_linux::{
        hashnt::Hashnt,
        wayland::{
            events::WaylandEvent,
            interfaces::all::*,
            methods::*,
            wire::{Fd, Id},
            WaylandClient,
        },
    },
    shm::{Buffer, ShmAllocator},
    std::{
        cell::{Cell, OnceCell},
        collections::{HashMap, VecDeque},
        env,
        fs::{File, OpenOptions},
        os::fd::AsRawFd,
        rc::Rc,
        thread,
        time::Duration,
    },
};

pub mod event_handler;
pub mod shm;

pub struct WaylandBackend {
    pub client: WaylandClient,
    pub event_queue: VecDeque<Event>,
    pub windows: Vec<Option<WaylandWindow>>,
    pub object_to_window_map: HashMap<Id, WindowId, Hashnt>,
    pub shm: OnceCell<ShmAllocator>,
}

impl WaylandBackend {
    pub fn new() -> Option<Self> {
        let client = WaylandClient::new()?;

        let mut this = Self {
            client,
            event_queue: VecDeque::new(),
            windows: Vec::default(),
            object_to_window_map: HashMap::default(),
            shm: OnceCell::new(),
        };

        this.roundtrip();
        this.shm.set(ShmAllocator::new(1, &mut this.client)?);

        println!("Finished lokinit init");

        Some(this)
    }

    /// Sends a [`WlDisplayMethod::Sync`] event to the compositor, and blocks the thread until
    /// it completes. While blocking, it processes and queues events to be handled later.
    pub fn roundtrip(&mut self) {
        let finished = Rc::new(Cell::new(false));
        let finished_clone = finished.clone();
        let callback = WlCallback::new(
            &mut self.client,
            Box::new(move |_| finished_clone.set(true)),
        );

        self.client
            .call_method(&WlDisplay::global(), WlDisplayMethod::Sync(callback));

        while !finished.get() {
            while let Some(event) = self.client.next_event(true) {
                if let Some(event) = self.handle_event(event) {
                    self.event_queue.push_back(event);
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
    }
}

impl LokinitBackend for WaylandBackend {
    fn init() -> Self {
        Self::new().unwrap()
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        let compositor: WlCompositor = self.client.get_global();
        let wm_base: XdgWmBase = self.client.get_global();

        let wl_surface = compositor.create_surface(&mut self.client);
        let xdg_surface = wm_base.get_xdg_surface(&mut self.client, wl_surface);
        let xdg_toplevel = xdg_surface.get_toplevel(&mut self.client);

        let window = WaylandWindow {
            wl_surface,
            xdg_surface,
            xdg_toplevel,
            buffer: self.shm.get_mut().unwrap().allocate(&self.client, 1),
        };

        self.client
            .call_method(&xdg_toplevel, XdgToplevelMethod::SetTitle(builder.title));
        self.client.call_method(
            &xdg_surface,
            XdgSurfaceMethod::SetWindowGeometry(
                builder.position.x,
                builder.position.y,
                builder.size.width as _,
                builder.size.height as _,
            ),
        );

        let window_id = self.windows.len();
        self.windows.push(Some(window));
        self.object_to_window_map.insert(wl_surface.id, window_id);
        self.object_to_window_map.insert(xdg_surface.id, window_id);
        self.object_to_window_map.insert(xdg_toplevel.id, window_id);

        Ok(WindowHandle(window_id))
    }

    fn close_window(&mut self, handle: WindowHandle) {
        let window = self.windows[handle.0].take().unwrap();
        self.client
            .call_method(&window.xdg_toplevel, XdgToplevelMethod::Destroy);
        self.client
            .call_method(&window.xdg_surface, XdgSurfaceMethod::Destroy);
        self.client
            .call_method(&window.wl_surface, WlSurfaceMethod::Destroy);
        self.client
            .call_method(&window.buffer.wl_buffer(), WlBufferMethod::Destroy);
    }

    fn fetch_monitors(&mut self) -> Vec<Monitor> {
        todo!("Fetch monitors")
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            if let Some(event) = self.event_queue.pop_front() {
                return Some(event);
            }

            if let Some(event) = self.client.next_event(false) {
                if let Some(event) = self.handle_event(event) {
                    return Some(event);
                }
            }
        }
    }
    fn set_screen_mode(&mut self, _handle: WindowHandle, _screen_mode: ScreenMode) {
        todo!("Set screen mode")
    }

    #[cfg(feature = "opengl")]
    fn load_opengl_func(&mut self, _proc_name: *const std::ffi::c_char) -> *mut std::ffi::c_void {
        todo!()
    }

    #[cfg(feature = "opengl")]
    fn create_window_surface(
        &mut self,
        _window: WindowHandle,
        _cfg: crate::prelude::OpenGlConfig,
    ) -> super::WindowSurface {
        todo!()
    }

    #[cfg(feature = "opengl")]
    fn make_surface_active(&self, window: WindowHandle, surface: crate::native::WindowSurface) {
        todo!()
    }

    #[cfg(feature = "opengl")]
    fn flush_surface(&self, window: WindowHandle, surface: super::WindowSurface) {
        todo!()
    }

    #[cfg(feature = "opengl")]
    fn update_surface(&self, surface: crate::native::WindowSurface) {
        todo!()
    }
}

pub struct WaylandWindow {
    pub wl_surface: WlSurface,
    pub xdg_surface: XdgSurface,
    pub xdg_toplevel: XdgToplevel,
    pub buffer: Buffer,
}

type WindowSurface = ();
