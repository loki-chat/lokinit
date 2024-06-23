//! Lokinit's OpenGL API.

pub use crate::native::WindowSurface;
use crate::{
    lok::{self, LokinitBackend},
    prelude::WindowHandle,
};

impl WindowHandle {
    pub fn create_surface(&self, cfg: OpenGlConfig) -> WindowSurface {
        lok::with(|backend| backend.create_window_surface(*self, cfg))
    }

    #[cfg(feature = "opengl")]
    pub fn make_surface_active(&self, surface: crate::native::WindowSurface) {
        lok::with(|backend| backend.make_surface_active(*self, surface))
    }

    #[cfg(feature = "opengl")]
    pub fn flush_surface(&self, surface: crate::native::WindowSurface) {
        lok::with(|backend| backend.flush_surface(*self, surface))
    }
}

#[derive(Debug, Default)]
pub enum X11GlBackend {
    #[default]
    Glx = 1,
    Egl = 2,
}

#[derive(Debug, Default)]
pub struct OpenGlConfig {
    pub x11_gl_backend: X11GlBackend,
    pub x11_gl_backend_fallback: Option<X11GlBackend>,
}
