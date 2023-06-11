use std::path::PathBuf;

use crate::keycode::KeyCode;

#[derive(Clone, Debug)]
pub enum Event {
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
    Key(KeyCode),
    Char(char),
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
    ButtonPress(MouseButton),
    ButtonRelease(MouseButton),
    CursorMove(f64, f64),
    CursorIn(f64, f64),
    CursorOut(f64, f64),
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
