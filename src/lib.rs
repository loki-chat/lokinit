pub mod event;
#[cfg(feature = "opengl")]
pub mod gl;
pub mod keycode;
pub mod lok;
pub mod native;
pub mod window;

pub mod prelude {
    #[cfg(feature = "opengl")]
    pub use crate::gl::{OpenGlConfig, WindowSurface};
    pub use crate::{
        event::{Event, EventKind, KeyboardEvent, MouseButton, MouseEvent, TouchEvent, TouchPhase},
        keycode::KeyCode,
        lok::{self, Monitor, MonitorId},
        native::DefaultLokinitBackend,
        window::{WindowBorder, WindowBuilder, WindowHandle, WindowPos, WindowSize},
    };
}
