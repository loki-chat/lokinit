use crate::ffi;

#[repr(usize)]
#[allow(clippy::enum_clike_unportable_variant)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSEventMask {
    Any = usize::MAX,
    // TODO: Other event masks
}

#[repr(usize)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSBackingStoreType {
    #[deprecated]
    Retained = 0,
    #[deprecated]
    Nonretained = 1,
    Buffered = 2,
}

#[derive(Default)]
#[repr(transparent)]
pub struct NSWindowStyleMask(usize);
impl NSWindowStyleMask {
    pub const fn borderless(self) -> Self {
        Self(0)
    }
    pub const fn titled(mut self) -> Self {
        self.0 |= 1 << 0;
        self
    }
    pub const fn closable(mut self) -> Self {
        self.0 |= 1 << 1;
        self
    }
    pub const fn miniaturizable(mut self) -> Self {
        self.0 |= 1 << 2;
        self
    }
    pub const fn resizable(mut self) -> Self {
        self.0 |= 1 << 3;
        self
    }
    pub const fn unified_title_and_toolbar(mut self) -> Self {
        self.0 |= 1 << 12;
        self
    }
    pub const fn fullscreen(mut self) -> Self {
        self.0 |= 1 << 14;
        self
    }
    pub const fn full_size_content_view(mut self) -> Self {
        self.0 |= 1 << 15;
        self
    }
    pub const fn utility(mut self) -> Self {
        self.0 |= 1 << 4;
        self
    }
    pub const fn doc_modal(mut self) -> Self {
        self.0 |= 1 << 6;
        self
    }
    pub const fn non_activating_panel(mut self) -> Self {
        self.0 |= 1 << 7;
        self
    }
    pub const fn hud_window(mut self) -> Self {
        self.0 |= 1 << 13;
        self
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSRunLoopMode {
    Default,
    EventTracking,
    ModalPanel,
}
impl From<NSRunLoopMode> for ffi::NSRunLoopMode {
    fn from(val: NSRunLoopMode) -> Self {
        unsafe {
            match val {
                NSRunLoopMode::Default => ffi::NSDefaultRunLoopMode,
                NSRunLoopMode::EventTracking => ffi::NSEventTrackingRunLoopMode,
                NSRunLoopMode::ModalPanel => ffi::NSModalPanelRunLoopMode,
            }
        }
    }
}

#[repr(usize)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSEventType {
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    MouseMoved = 5,
    LeftMouseDragged = 6,
    RightMouseDragged = 7,
    MouseEntered = 8,
    MouseExited = 9,
    KeyDown = 10,
    KeyUp = 11,
    FlagsChanged = 12,
    AppKitDefined = 13,
    SystemDefined = 14,
    ApplicationDefined = 15,
    Periodic = 16,
    CursorUpdate = 17,
    ScrollWheel = 22,
    TabletPoint = 23,
    TabletProximity = 24,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    OtherMouseDragged = 27,
    Gesture = 29,
    Magnify = 30,
    Swipe = 31,
    Rotate = 18,
    BeginGesture = 19,
    EndGesture = 20,
    Pressure = 34,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSEventSubtype {
    ApplicationActivated,
    ApplicationDeactivated,
    ScreenChanged,
    WindowExposed,
    WindowMoved,
    Undocumented(i16),
}
impl From<i16> for NSEventSubtype {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::WindowExposed,
            // TODO: Can technically also be the "system power off" event, not sure how to handle that
            1 => Self::ApplicationActivated,
            2 => Self::ApplicationDeactivated,
            4 => Self::WindowMoved,
            8 => Self::ScreenChanged,
            other => Self::Undocumented(other),
        }
    }
}

#[repr(isize)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSApplicationActivationPolicy {
    Regular,
    Accessory,
    Prohibited,
}
