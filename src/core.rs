#![allow(unused)]

use crate::event::Event;
use crate::window::{WindowBuilder, WindowHandle, WindowPos, WindowSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonitorId(usize);

#[derive(Clone, Debug)]
pub struct Monitor {
    id: MonitorId,
    position: (u32, u32),
    size: (u32, u32),
    hertz: u32,
}

pub struct LokinitCore;

impl LokinitCore {
    fn init() -> Self {
        todo!()
    }

    fn fetch_monitors() -> Vec<Monitor> {
        todo!()
    }

    fn create_window(&mut self, builder: WindowBuilder) -> WindowHandle {
        todo!()
    }

    fn poll_event(&mut self) -> Option<(WindowHandle, Event)> {
        todo!()
    }

    fn window_pos(&self, window: WindowHandle) -> WindowPos {
        todo!()
    }

    fn window_size(&self, window: WindowHandle) -> WindowSize {
        todo!()
    }
}
