#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WindowPos {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WindowSize {
    width: u32,
    height: u32,
}

#[derive(Clone, Debug, Default)]
pub struct WindowBuilder {
    title: String,
    position: WindowPos,
    size: WindowSize,
    resizable: bool,
    maximized: bool,
    fullscreen: bool,
    transparent: bool,
    high_dpi: bool,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.position = WindowPos { x, y };
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = WindowSize { width, height };
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
pub struct WindowHandle(usize);
