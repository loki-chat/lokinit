use loki_linux::glx::GLXContext;

#[derive(Clone, Copy)]
pub struct GlSurface(pub(crate) GLXContext);
