use loki_linux::glx::GLXContext;

use super::OpenGlSurface;

pub struct GlxSurface(pub(crate) GLXContext);

impl OpenGlSurface for GlxSurface {
    fn make_active(&self) {
        todo!()
    }
    fn flush(&self) {
        todo!()
    }
}
