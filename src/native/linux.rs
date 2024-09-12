use crate::event::Event;
use crate::lok::{CreateWindowError, LokinitBackend};
use crate::window::{ScreenMode, WindowBuilder, WindowHandle};

use wayland::WaylandBackend;
use x11::X11Backend;

#[cfg(feature = "opengl")]
use crate::gl::*;

#[cfg(feature = "opengl")]
pub mod opengl;

pub mod wayland;
pub mod x11;

pub enum LinuxBackend {
    X11(X11Backend),
    Wayland(WaylandBackend),
}

impl LokinitBackend for LinuxBackend {
    fn init() -> Self {
        match std::env::var("LOKINIT_BACKEND") {
            Ok(x) if x == "wayland" => Self::Wayland(WaylandBackend::init()),
            Ok(x) if x == "x11" => Self::X11(X11Backend::init().unwrap()),

            _ => match WaylandBackend::new() {
                Ok(x) => Self::Wayland(x),
                Err(why) => {
                    eprintln!(
                        "Lokinit: Failed to initialize Wayland backend, falling back on X11\n\n{why}"
                    );
                    Self::X11(X11Backend::init().unwrap())
                }
            },
        }
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        match self {
            Self::X11(x11) => x11.create_window(builder),
            Self::Wayland(wl) => wl.create_window(builder),
        }
    }

    fn close_window(&mut self, handle: WindowHandle) {
        match self {
            Self::X11(x11) => x11.close_window(handle),
            Self::Wayland(wl) => wl.close_window(handle),
        }
    }

    fn poll_event(&mut self) -> Option<Event> {
        match self {
            Self::X11(x11) => x11.poll_event(),
            Self::Wayland(wl) => wl.poll_event(),
        }
    }

    fn set_screen_mode(&mut self, handle: WindowHandle, screen_mode: ScreenMode) {
        match self {
            Self::X11(x11) => x11.set_screen_mode(handle, screen_mode),
            Self::Wayland(wl) => wl.set_screen_mode(handle, screen_mode),
        }
    }

    fn fetch_monitors(&mut self) -> Vec<crate::prelude::Monitor> {
        todo!("fetch monitors")
    }

    #[cfg(feature = "opengl")]
    fn create_window_surface(
        &mut self,
        handle: WindowHandle,
        config: OpenGlConfig,
    ) -> WindowSurface {
        match self {
            Self::X11(x11) => x11.create_window_surface(handle, config),
            Self::Wayland(wl) => wl.create_window_surface(handle, config),
        }
    }

    #[cfg(feature = "opengl")]
    fn load_opengl_func(&mut self, proc_name: *const std::ffi::c_char) -> *mut std::ffi::c_void {
        match self {
            Self::X11(x11) => x11.load_opengl_func(proc_name),
            Self::Wayland(wl) => wl.load_opengl_func(proc_name),
        }
    }

    #[cfg(feature = "opengl")]
    fn make_surface_active(&self, handle: WindowHandle, surface: super::WindowSurface) {
        match self {
            Self::X11(x11) => x11.make_surface_active(handle, surface),
            Self::Wayland(wl) => wl.make_surface_active(handle, surface),
        }
    }

    #[cfg(feature = "opengl")]
    fn flush_surface(&self, handle: WindowHandle, surface: super::WindowSurface) {
        match self {
            Self::X11(x11) => x11.flush_surface(handle, surface),
            Self::Wayland(wl) => wl.flush_surface(handle, surface),
        }
    }

    #[cfg(feature = "opengl")]
    fn update_surface(&self, surface: super::WindowSurface) {
        todo!()
    }
}
