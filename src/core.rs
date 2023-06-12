#![allow(unused)]

use crate::event::Event;
use crate::native;
use crate::native::linux::x11::X11NativeCore;
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

pub struct LokinitCore(#[cfg(target_os = "linux")] native::linux::x11::X11NativeCore);

#[cfg(target_os = "linux")]
impl LokinitCore {
    pub fn init() -> Result<Self, native::linux::x11::NativeCoreError> {
        let native_core = unsafe { X11NativeCore::init() }?;
        Ok(Self(native_core))
    }

    pub fn fetch_monitors() -> Vec<Monitor> {
        todo!()
    }

    pub fn create_window(
        &mut self,
        builder: WindowBuilder,
    ) -> Result<WindowHandle, native::linux::x11::CreateWindowError> {
        unsafe { self.0.create_window(builder) }
    }

    pub fn poll_event(&mut self) -> Option<(WindowHandle, Event)> {
        unsafe { self.0.poll_event() }
    }

    pub fn window_pos(&self, window: WindowHandle) -> WindowPos {
        self.0.window_pos(window)
    }

    pub fn window_size(&self, window: WindowHandle) -> WindowSize {
        self.0.window_size(window)
    }
}
