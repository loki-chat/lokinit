use {
    crate::{gl::OpenGLSurface, prelude::WindowHandle},
    loki_mac::ffi::NSOpenGLContext,
};

pub struct WindowSurface {
    pub(crate) context: NSOpenGLContext,
    pub(crate) window: WindowHandle,
}
impl OpenGLSurface for WindowSurface {
    fn make_active(&self) {
        self.context.make_current();
    }
    fn flush(&self) {
        // TODO: Update only really needs to be called when the view is resized.
        self.context.update();
        self.context.flush_buffer();
    }
}
