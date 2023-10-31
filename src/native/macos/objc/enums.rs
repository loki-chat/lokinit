use super::macros::composable_enum;

/// https://developer.apple.com/documentation/foundation/nsstringencoding?language=objc
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

#[repr(usize)]
pub enum NSBackingStoreType {
    Retained = 0,
    Nonretained = 1,
    Buffered = 2,
}

composable_enum!(
    @usize:
    NSWindowStyleMask
        Borderless = 0,
        Titled = 1 << 0,
        Closable = 1 << 1,
        Miniaturizable = 1 << 2,
        Resizable = 1 << 3,
        TexturedBackground = 1 << 8,
        UnifiedTitleAndToolbar = 1 << 12,
        FullScreen = 1 << 14,
        FullSizeContentView = 1 << 15,
        UtilityWindow = 1 << 4,
        DocModalWindow = 1 << 6,
        NonactivatingPanel = 1 << 7,
        HUDPanel = 1 << 13
);

#[repr(isize)]
pub enum NSApplicationActivationPolicy {
    Regular = 0,
    Accessory = 1,
    Prohibited = 2,
}

/// https://developer.apple.com/documentation/appkit/nseventtype?language=objc
#[repr(usize)]
#[derive(PartialEq, Debug)]
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

/// https://developer.apple.com/documentation/appkit/nseventsubtype?language=objc
#[repr(i16)]
pub enum NSEventSubtype {
    ApplicationActivated = 1,
    ApplicationDeactivated = 2,
    ScreenChanged = 8,
    WindowExposed = 0,
    WindowMoved = 4,
    // This will cause issues since it's the same number, and we don't use it, but technically SystemPowerOff is also 1.
    // PowerOff = 1,
}

/// https://developer.apple.com/documentation/appkit/nswindowbutton?language=objc
/// The docs don't list constant values for this enum's variants... in theory, they'll both
/// start at 0 and just count up, but if it's ever not this could cause issues in the future.
#[repr(usize)]
pub enum NSWindowButton {
    Close,
    Miniaturize,
    Zoom,
    Toolbar,
    DocumentIcon,
    DocumentVersions,
    // Deprecated
    // FullScreen
}
