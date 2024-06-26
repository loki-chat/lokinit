use crate::{
    lok::{self, LokinitBackend, MonitorId},
    native::WindowId,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WindowPos {
    pub x: i32,
    pub y: i32,
}

impl WindowPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl WindowSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowBorder {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,

    TopLeft = 4,
    TopRight = 5,
    BottomLeft = 6,
    BottomRight = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ScreenMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
    ExclusiveFullscreen,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowBuilder {
    pub title: String,
    pub position: WindowPos,
    pub size: WindowSize,
    pub monitor: Option<MonitorId>,
    pub screen_mode: ScreenMode,
    pub centered: bool,
    pub resizable: bool,
    pub maximized: bool,
    pub transparent: bool,
    pub high_dpi: bool,
    pub decorations: bool,
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

    pub fn decorations(mut self, has_decorations: bool) -> Self {
        self.decorations = has_decorations;
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
impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: String::new(),
            position: WindowPos { x: 200, y: 400 },
            size: WindowSize {
                width: 600,
                height: 400,
            },
            monitor: None,
            screen_mode: ScreenMode::Windowed,
            centered: false,
            resizable: true,
            maximized: false,
            decorations: true,
            transparent: false,
            high_dpi: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowHandle(pub(crate) WindowId);
impl WindowHandle {
    pub fn close(self) {
        lok::with(|backend| {
            backend.close_window(self);
        });
    }
}
