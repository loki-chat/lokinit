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
    pub fn init() -> Self {
        todo!()
    }

    pub fn fetch_monitors() -> Vec<Monitor> {
        todo!()
    }

    pub fn create_window(&mut self, builder: WindowBuilder) -> WindowHandle {
        todo!()
    }

    pub fn poll_event(&mut self) -> Option<(WindowHandle, Event)> {
        todo!()
    }

    pub fn window_pos(&self, window: WindowHandle) -> WindowPos {
        todo!()
    }

    pub fn window_size(&self, window: WindowHandle) -> WindowSize {
        todo!()
    }
}
