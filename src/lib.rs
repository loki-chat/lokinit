pub mod core;
pub mod event;
pub mod keycode;
mod native;
pub mod window;

pub mod prelude {
    pub use crate::core::{self, Monitor, MonitorId};
    pub use crate::event::{
        Event, EventKind, KeyboardEvent, MouseButton, MouseEvent, TouchEvent, TouchPhase,
    };
    pub use crate::keycode::KeyCode;
    pub use crate::lok;
    pub use crate::native::DefaultLokinitBackend;
    pub use crate::window::{WindowBuilder, WindowHandle, WindowPos, WindowSize};
}

pub mod lok {
    use crate::core;
    use crate::native::DefaultLokinitBackend;

    /// Initializes Lokinit with a default backend.
    pub fn init() {
        core::init::<DefaultLokinitBackend>();
    }
}
