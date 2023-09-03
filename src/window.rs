use crate::lok::MonitorId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WindowPos {
    pub x: i32,
    pub y: i32,
}

impl WindowPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ScreenMode {
    #[default]
    Windowed,
    Borderless,
    Fullscreen,
}

#[derive(Clone, Debug, Default)]
pub struct WindowBuilder {
    pub(crate) title: String,
    pub(crate) position: WindowPos,
    pub(crate) size: WindowSize,
    pub(crate) monitor: Option<MonitorId>,
    pub(crate) screen_mode: ScreenMode,
    pub(crate) centered: bool,
    pub(crate) resizable: bool,
    pub(crate) maximized: bool,
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

    pub fn screen_mode(mut self, screen_mode: ScreenMode) -> Self {
        self.screen_mode = screen_mode;
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

#[cfg(feature = "raw-window-handle")]
unsafe impl raw_window_handle::HasRawWindowHandle for WindowHandle {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        crate::lok::raw_window_handle(*self)
    }
}

#[cfg(feature = "raw-window-handle")]
unsafe impl raw_window_handle::HasRawDisplayHandle for WindowHandle {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        crate::lok::raw_display_handle()
    }
}
