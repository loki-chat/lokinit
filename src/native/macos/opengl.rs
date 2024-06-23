use crate::{
    gl::OpenGLSurface,
    lok::{self, LokinitBackend},
    prelude::WindowHandle,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WindowSurface {
    pub(crate) window: WindowHandle,
}
impl OpenGLSurface for WindowSurface {
    fn make_active(&self) {
        lok::with(|backend| {
            backend.make_surface_active(*self);
        });
    }
    fn flush(&self) {
        lok::with(|backend| {
            backend.flush_surface(*self);
        });
    }
}
