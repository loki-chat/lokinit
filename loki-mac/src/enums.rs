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

#[repr(usize)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NSEventModifierFlags {
    /// Caps lock was pressed.
    CapsLock = 1 << 16,
    /// Shift was pressed.
    Shift = 1 << 17,
    /// Control was pressed.
    Control = 1 << 18,
    /// Option or Alt was pressed.
    Option = 1 << 19,
    /// Command (Windows key) was pressed.
    Command = 1 << 20,
    /// A number keypad key or arrow key was pressed.
    NumericPad = 1 << 21,
    /// The Help key was pressed.
    Help = 1 << 22,
    /// A function key was pressed.
    Function = 1 << 23,
}

#[repr(usize)]
pub enum NSStringEncoding {
    ASCII = 1,
    NEXTSTEP = 2,
    JapaneseEUC = 3,
    UTF8 = 4,
    ISOLatin1 = 5,
    AdobeSymbol = 6,
    NonLossyASCII = 7,
    ShiftJIS = 8,
    Latin2 = 9,
    Unicode = 10,
    WindowsCP1251 = 11,
    WindowsCP1252 = 12,
    WindowsCP1253 = 13,
    WindowsCP1254 = 14,
    WindowsCP1250 = 15,
    ISO2022JP = 21,
    MacOSRoman = 30,
    // UTF16 is the same as Unicode, which will cause an error in Rust, but it's documented here
    // just to match the enum 1:1.
    // UTF16 = 10,
    UTF16BigEndian = 0x90000100,
    UTF16LittleEndian = 0x94000100,
    UTF32 = 0x8c000100,
    UTF32BigEndian = 0x98000100,
    UTF32LittleEndian = 0x9c000100,
    // Deprecated, but documented to match the enum 1:1.
    // ProprietaryString = 65536
}

#[repr(u32)]
pub enum NSOpenGLPFA {
    Accelerated = 73,
    AcceleratedCompute = 97,
    AccumSize = 14,
    AllRenderers = 1,
    AllowOfflineRenderers = 96,
    AlphaSize = 11,
    AuxBuffers = 7,
    AuxDepthStencil = 57,
    BackingStore = 76,
    ClosestPolicy = 74,
    ColorFloat = 58,
    ColorSize = 8,
    Compliant = 83,
    DepthSize = 12,
    DoubleBuffer = 5,
    FullScreen = 54,
    MPSafe = 78,
    MaximumPolicy = 52,
    MinimumPolicy = 51,
    MultiScreen = 81,
    Multisample = 59,
    NoRecovery = 72,
    OffScreen = 53,
    OpenGLProfile = 99,
    PixelBuffer = 90,
    RemotePixelBuffer = 91,
    RendererID = 70,
    Robust = 75,
    SampleAlpha = 61,
    SampleBuffers = 55,
    Samples = 56,
    ScreenMask = 84,
    SingleRenderer = 71,
    StencilSize = 13,
    Stereo = 6,
    Supersample = 60,
    TripleBuffer = 3,
    VirtualScreenCount = 128,
    Window = 80,
}
#[repr(u32)]
pub enum NSOpenGLProfile {
    /// Pre-OpenGL 3.0
    Legacy = 0x1000,
    Core3_2 = 0x3200,
    Core4_1 = 0x4100,
}
