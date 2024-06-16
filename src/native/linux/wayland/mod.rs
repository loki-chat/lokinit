use {
    self::{display_handler::DisplayEventListener, registry_handler::RegistryEventListener},
    crate::{
        event::Event,
        lok::{CreateWindowError, LokinitBackend},
        prelude::{Monitor, WindowBuilder, WindowHandle},
        window::ScreenMode,
    },
    loki_linux::wayland::{
        interfaces::{
            callback::Callback, compositor::Compositor, display::WaylandDisplay,
            registry::WaylandRegistry, xdg::XdgWmBase,
        },
        WaylandClient,
    },
    std::{
        cell::{Cell, RefCell},
        rc::Rc,
        thread,
        time::Duration,
    },
};

#[cfg(feature = "opengl")]
use crate::gl::*;

pub mod display_handler;
pub mod registry_handler;

pub struct WaylandBackend {
    client: WaylandClient,
    state: Rc<RefCell<WaylandState>>,
}

#[derive(Default)]
pub struct WaylandState {
    pub compositor: Option<Compositor>,
    pub xdg_wm_base: Option<XdgWmBase>,
}

impl WaylandBackend {
    pub fn new() -> Option<Self> {
        let client = WaylandClient::new()?;
        let state = Rc::new(RefCell::new(WaylandState::default()));
        let this = Self {
            client,
            state: state.clone(),
        };

        WaylandDisplay.set_listener(DisplayEventListener {});
        WaylandRegistry.set_listener(RegistryEventListener { state });

        this.block_until_next_event();

        Some(this)
    }

    pub fn block_until_next_event(&self) {
        let finished = Rc::new(Cell::new(false));
        let finished_clone = finished.clone();
        WaylandDisplay.sync(Callback::new(move |_| finished_clone.set(true)));

        while !finished.get() {
            thread::sleep(Duration::from_millis(5));
            self.client.read_message();
        }

        println!("Finished [S Y N C]")
    }
}

impl LokinitBackend for WaylandBackend {
    fn init() -> Self {
        Self::new().unwrap()
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        todo!("Create window")
    }

    fn close_window(&mut self, handle: WindowHandle) {
        todo!("Close window")
    }

    fn fetch_monitors(&mut self) -> Vec<Monitor> {
        todo!("Fetch monitors")
    }

    fn poll_event(&mut self) -> Option<Event> {
        todo!("Poll event")
    }

    fn set_screen_mode(&mut self, handle: WindowHandle, screen_mode: ScreenMode) {
        todo!("Set screen mode")
    }

    fn create_window_surface(
        &mut self,
        window: WindowHandle,
        cfg: crate::prelude::OpenGLConfig,
    ) -> GLSurface {
        todo!("create window surface")
    }

    fn load_opengl_func(
        &mut self,
        proc_name: *const std::ffi::c_char,
    ) -> Option<*mut std::ffi::c_void> {
        todo!("load OpenGL function")
    }
}
