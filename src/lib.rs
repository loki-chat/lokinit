pub mod core;
pub mod event;
pub mod keycode;
mod native;
pub mod window;

pub mod prelude {
    pub use crate::core::{LokinitCore, Monitor, MonitorId};
    pub use crate::event::{Event, KeyboardEvent, MouseButton, MouseEvent, TouchEvent, TouchPhase};
    pub use crate::keycode::KeyCode;
    pub use crate::window::{WindowBuilder, WindowHandle, WindowPos, WindowSize};
}
