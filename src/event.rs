use std::path::PathBuf;
use std::time::Duration;

use crate::keycode::KeyCode;
use crate::window::WindowHandle;

#[derive(Clone, Debug)]
pub struct Event {
    pub time: Duration,
    pub window: WindowHandle,
    pub kind: EventKind,
}

#[derive(Clone, Debug)]
pub enum EventKind {
    Resized(u32, u32),
    Moved(i32, i32),

    CloseRequested,
    Destroyed,

    FileDropped(PathBuf),
    FileHovered(PathBuf),
    FileHoveredCancelled(PathBuf),

    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    Touch(TouchEvent),

    Redraw,

    FocusIn,
    FocusOut,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KeyboardEvent {
    KeyPress(KeyCode),
    KeyRelease(KeyCode),
    KeyRepeat(KeyCode),
    ImeCommit(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    ButtonPress(MouseButton, i32, i32),
    ButtonRelease(MouseButton, i32, i32),
    CursorMove(i32, i32),
    CursorIn(i32, i32),
    CursorOut(i32, i32),
    Wheel(f64, f64),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TouchEvent {
    phase: TouchPhase,
    x: f64,
    y: f64,
}
