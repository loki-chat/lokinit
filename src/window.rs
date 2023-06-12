use crate::core::MonitorId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WindowPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default)]
pub struct WindowBuilder {
    pub(crate) title: String,
    pub(crate) position: WindowPos,
    pub(crate) size: WindowSize,
    pub(crate) monitor: Option<MonitorId>,
    pub(crate) centered: bool,
    pub(crate) resizable: bool,
    pub(crate) maximized: bool,
    pub(crate) fullscreen: bool,
    pub(crate) transparent: bool,
    pub(crate) high_dpi: bool,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = WindowPos { x, y };
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = WindowSize { width, height };
        self
    }

    pub fn monitor(mut self, monitor: MonitorId) -> Self {
        self.monitor = Some(monitor);
        self
    }

    pub fn centered(mut self, is_centered: bool) -> Self {
        self.centered = is_centered;
        self
    }

    pub fn resizable(mut self, is_resizable: bool) -> Self {
        self.resizable = is_resizable;
        self
    }

    pub fn maximized(mut self, is_maximized: bool) -> Self {
        self.maximized = is_maximized;
        self
    }

    pub fn fullscreen(mut self, is_fullscreen: bool) -> Self {
        self.fullscreen = is_fullscreen;
        self
    }

    pub fn transparent(mut self, is_trans: bool) -> Self {
        self.transparent = is_trans;
        self
    }

    pub fn high_dpi(mut self, is_enabled: bool) -> Self {
        self.high_dpi = is_enabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle(pub(crate) usize);
