pub mod event;
#[cfg(feature = "opengl")]
pub mod gl;
pub mod keycode;
pub mod lok;
mod native;
pub mod window;

pub mod prelude {
    pub use crate::event::{
        Event, EventKind, KeyboardEvent, MouseButton, MouseEvent, TouchEvent, TouchPhase,
    };
    #[cfg(feature = "opengl")]
    pub use crate::gl::{OpenGLConfig, OpenGLSurface};
    pub use crate::keycode::KeyCode;
    pub use crate::lok::{self, Monitor, MonitorId};
    pub use crate::native::DefaultLokinitBackend;
    pub use crate::window::{WindowBuilder, WindowHandle, WindowPos, WindowSize};
}
