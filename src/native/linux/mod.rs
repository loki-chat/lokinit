use crate::event::Event;
use crate::lok::{CreateWindowError, LokinitBackend};
use crate::window::{WindowBuilder, WindowHandle, ScreenMode};

use wayland::WaylandBackend;
use x11::X11Backend;

pub mod wayland;
pub mod x11;

pub enum LinuxBackend {
    X11(X11Backend),
    Wayland(WaylandBackend),
}

impl LokinitBackend for LinuxBackend {
    fn init() -> Self
    where
        Self: Sized + 'static,
    {
        // TODO: wayland backend
        Self::X11(X11Backend::init().unwrap())
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
        match self {
            Self::X11(x11) => x11.create_window(builder),
            Self::Wayland(_wl) => todo!(),
        }
    }

    fn close_window(&mut self, handle: WindowHandle) {
        match self {
            Self::X11(x11) => x11.close_window(handle),
            Self::Wayland(_wl) => todo!(),
        }
    }

    fn poll_event(&mut self) -> Option<Event> {
        match self {
            Self::X11(x11) => x11.poll_event(),
            Self::Wayland(_wl) => todo!(),
        }
    }

    #[cfg(feature = "raw-window-handle")]
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        match self {
            Self::X11(_x11) => todo!(),
            Self::Wayland(_wl) => todo!(),
        }
    }

    #[cfg(feature = "raw-window-handle")]
    fn raw_window_handle_for(&self, window_handle: WindowHandle) -> raw_window_handle::RawWindowHandle {
        match self {
            Self::X11(_x11) => todo!(),
            Self::Wayland(_wl) => todo!(),
        }
    }

    fn set_screen_mode(&mut self, handle: WindowHandle, screen_mode: ScreenMode) {
        match self {
            Self::X11(x11) => x11.set_screen_mode(handle, screen_mode),
            Self::Wayland(_wl) => todo!(),
        }
    }
}
