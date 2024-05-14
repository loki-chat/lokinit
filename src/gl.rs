//! Lokinit's OpenGL API.

pub use crate::native::GLSurface;
use crate::{
    lok::{self},
    prelude::WindowHandle,
};

pub trait OpenGLSurface {
    fn make_active(&self);
    fn flush(&self);
}

impl WindowHandle {
    pub fn create_surface(&self, cfg: OpenGLConfig) -> GLSurface {
        lok::with(|backend| backend.create_window_surface(*self, cfg))
    }
}

pub struct OpenGLConfig {}
