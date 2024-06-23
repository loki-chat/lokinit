//! Lokinit's OpenGL API.

pub use crate::native::GLSurface;
use crate::{
    lok::{self},
    prelude::WindowHandle,
};

pub trait OpenGlSurface {
    fn make_active(&self);
    fn flush(&self);
}

impl WindowHandle {
    pub fn create_surface(&self, cfg: OpenGlConfig) -> GLSurface {
        lok::with(|backend| backend.create_window_surface(*self, cfg))
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
