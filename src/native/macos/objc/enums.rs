use crate::composable_enum;

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

/// 1:1 representation of the
/// [Objective-C NSEventType enum](https://developer.apple.com/documentation/appkit/nseventtype?language=objc)
#[repr(usize)]
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

/// 1:1 representation of the
/// [Objective-C NSEventSubtype enum](https://developer.apple.com/documentation/appkit/nseventsubtype?language=objc)
#[repr(i16)]
pub enum NSEventSubtype {
    ApplicationActivated = 1,
    ApplicationDeactivated = 2,
    ScreenChanged = 8,
    WindowExposed = 0,
    WindowMoved = 4,
    // This will cause issues with Rust and we don't use it, but technically System PowerOff is 1.
    // PowerOff = 1,
}
