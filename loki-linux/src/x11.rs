#![allow(non_snake_case, clippy::upper_case_acronyms)]

use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};

pub mod xevents;

pub use xevents::*;

use crate::library;

#[repr(C)]
pub struct XDisplay([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct XID(pub(crate) c_ulong);

impl XID {
    /// Creates a [`XID`] from a raw ID.
    ///
    /// # Safety
    ///
    /// Make sure this is a valid X11 window ID.
    pub unsafe fn from_raw(id: c_ulong) -> Self {
        Self(id)
    }

    /// Returns the raw ID of this [`XID`].
    pub fn raw(&self) -> c_ulong {
        self.0
    }
}

pub type Drawable = XID;
pub type VisualID = XID;
pub type XPointer = *mut c_char;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Pixmap = XID;
pub type KeySym = XID;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XWindow(pub(crate) c_ulong);

impl XWindow {
    pub const NONE: Self = XWindow(0);

    /// Creates an [`XWindow`] from a raw ID.
    ///
    /// # Safety
    ///
    /// Make sure this is a valid X11 window ID.
    pub unsafe fn from_raw(id: c_ulong) -> Self {
        Self(id)
    }

    /// Returns the raw ID of this [`XWindow`].
    pub fn raw(&self) -> c_ulong {
        self.0
    }
}

pub type Status = c_int;
pub type Time = c_ulong;
pub type Atom = c_ulong;
pub type Bool = c_int;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XPoint {
    x: c_short,
    y: c_short,
}

impl XPoint {
    pub fn new(x: c_short, y: c_short) -> Self {
        Self { x, y }
    }
}

pub type XVaNestedList = *mut c_void;

pub const X_BUFFER_OVERFLOW: c_int = -1;
pub const X_LOOKUP_NONE: c_int = 1;
pub const X_LOOKUP_CHARS: c_int = 2;
pub const X_LOOKUP_KEY_SYM: c_int = 3;
pub const X_LOOKUP_BOTH: c_int = 4;

#[repr(C)]
pub struct _XrmHashBucketRec([u8; 0]);
pub type XrmDatabase = *mut _XrmHashBucketRec;

#[repr(C)]
pub struct _XIM([u8; 0]);
pub type XIM = *mut _XIM;

#[repr(C)]
pub struct _XIC([u8; 0]);
pub type XIC = *mut _XIC;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XComposeStatus {
    pub compose_ptr: XPointer,
    pub chars_matched: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XErrorEvent {
    pub type_: c_int,
    pub display: *mut XDisplay,
    pub resourceid: XID,
    pub serial: c_ulong,
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
}

pub type XErrorHandler = unsafe extern "C" fn(*mut XDisplay, *mut XErrorEvent) -> c_int;

#[repr(C)]
pub struct XExtData {
    pub number: c_int,
    pub next: *mut XExtData,
    pub free_private: Option<unsafe extern "C" fn() -> c_int>,
    pub private_data: XPointer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub bits_per_rgb: c_int,
    pub map_entries: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: Bool,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: Bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

pub mod errcode {
    use std::ffi::c_int;

    pub const SUCCESS: c_int = 0;
    pub const BAD_REQUEST: c_int = 1;
    pub const BAD_VALUE: c_int = 2;
    pub const BAD_WINDOW: c_int = 3;
    pub const BAD_PIXMAP: c_int = 4;
    pub const BAD_ATOM: c_int = 5;
    pub const BAD_CURSOR: c_int = 6;
    pub const BAD_FONT: c_int = 7;
    pub const BAD_MATCH: c_int = 8;
    pub const BAD_DRAWABLE: c_int = 9;
    pub const BAD_ACCESS: c_int = 10;
    pub const BAD_ALLOC: c_int = 11;
    pub const BAD_COLOR: c_int = 12;
    pub const BAD_GC: c_int = 13;
    pub const BAD_ID_CHOICE: c_int = 14;
    pub const BAD_NAME: c_int = 15;
    pub const BAD_LENGTH: c_int = 16;
    pub const BAD_IMPLEMENTATION: c_int = 17;
}

pub mod xclass {
    use std::ffi::c_uint;

    pub const INPUT_OUTPUT: c_uint = 1;
    pub const INPUT_ONLY: c_uint = 2;
}

pub mod xcw {
    use std::ffi::c_ulong;

    pub const BACK_PIXMAP: c_ulong = 0x0001;
    pub const BACK_PIXEL: c_ulong = 0x0002;
    pub const BORDER_PIXMAP: c_ulong = 0x0004;
    pub const BORDER_PIXEL: c_ulong = 0x0008;
    pub const BIT_GRAVITY: c_ulong = 0x0010;
    pub const WIN_GRAVITY: c_ulong = 0x0020;
    pub const BACKING_STORE: c_ulong = 0x0040;
    pub const BACKING_PLANES: c_ulong = 0x0080;
    pub const BACKING_PIXEL: c_ulong = 0x0100;
    pub const OVERRIDE_REDIRECT: c_ulong = 0x0200;
    pub const SAVE_UNDER: c_ulong = 0x0400;
    pub const EVENT_MASK: c_ulong = 0x0800;
    pub const DONT_PROPAGATE: c_ulong = 0x1000;
    pub const COLORMAP: c_ulong = 0x2000;
    pub const CURSOR: c_ulong = 0x4000;
}

pub mod xevent_mask {
    use std::ffi::c_long;

    pub const NO_EVENT: c_long = 0;
    pub const KEY_PRESS: c_long = 0x0000_0001;
    pub const KEY_RELEASE: c_long = 0x0000_0002;
    pub const BUTTON_PRESS: c_long = 0x0000_0004;
    pub const BUTTON_RELEASE: c_long = 0x0000_0008;
    pub const ENTER_WINDOW: c_long = 0x0000_0010;
    pub const LEAVE_WINDOW: c_long = 0x0000_0020;
    pub const POINTER_MOTION: c_long = 0x0000_0040;
    pub const POINTER_MOTION_HINT: c_long = 0x0000_0080;
    pub const BUTTON1_MOTION: c_long = 0x0000_0100;
    pub const BUTTON2_MOTION: c_long = 0x0000_0200;
    pub const BUTTON3_MOTION: c_long = 0x0000_0400;
    pub const BUTTON4_MOTION: c_long = 0x0000_0800;
    pub const BUTTON5_MOTION: c_long = 0x0000_1000;
    pub const BUTTON_MOTION: c_long = 0x0000_2000;
    pub const KEYMAP_STATE: c_long = 0x0000_4000;
    pub const EXPOSURE: c_long = 0x0000_8000;
    pub const VISIBILITY_CHANGE: c_long = 0x0001_0000;
    pub const STRUCTURE_NOTIFY: c_long = 0x0002_0000;
    pub const RESIZE_REDIRECT: c_long = 0x0004_0000;
    pub const SUBSTRUCTURE_NOTIFY: c_long = 0x0008_0000;
    pub const SUBSTRUCTURE_REDIRECT: c_long = 0x0010_0000;
    pub const FOCUS_CHANGE: c_long = 0x0020_0000;
    pub const PROPERTY_CHANGE: c_long = 0x0040_0000;
    pub const COLORMAP_CHANGE: c_long = 0x0080_0000;
    pub const OWNER_GRAB_BUTTON: c_long = 0x0100_0000;
}

pub mod xim {
    use std::ffi::c_long;

    pub const PREEDIT_AREA: c_long = 0x0001;
    pub const PREEDIT_CALLBACKS: c_long = 0x0002;
    pub const PREEDIT_POSITION: c_long = 0x0004;
    pub const PREEDIT_NOTHING: c_long = 0x0008;
    pub const PREEDIT_NONE: c_long = 0x0010;
    pub const STATUS_AREA: c_long = 0x0100;
    pub const STATUS_CALLBACKS: c_long = 0x0200;
    pub const STATUS_NOTHING: c_long = 0x0400;
    pub const STATUS_NONE: c_long = 0x0800;
}

#[rustfmt::skip]
pub mod xn {
    use std::ffi::c_char;

    pub const VA_NESTED_LIST:                *const c_char = b"XNVaNestedList\0"             .as_ptr() as *const _;
    pub const QUERY_INPUT_STYLE:             *const c_char = b"queryInputStyle\0"            .as_ptr() as *const _;
    pub const CLIENT_WINDOW:                 *const c_char = b"clientWindow\0"               .as_ptr() as *const _;
    pub const INPUT_STYLE:                   *const c_char = b"inputStyle\0"                 .as_ptr() as *const _;
    pub const FOCUS_WINDOW:                  *const c_char = b"focusWindow\0"                .as_ptr() as *const _;
    pub const RESOURCE_NAME:                 *const c_char = b"resourceName\0"               .as_ptr() as *const _;
    pub const RESOURCE_CLASS:                *const c_char = b"resourceClass\0"              .as_ptr() as *const _;
    pub const GEOMETRY_CALLBACK:             *const c_char = b"geometryCallback\0"           .as_ptr() as *const _;
    pub const DESTROY_CALLBACK:              *const c_char = b"destroyCallback\0"            .as_ptr() as *const _;
    pub const FILTER_EVENTS:                 *const c_char = b"filterEvents\0"               .as_ptr() as *const _;
    pub const PREEDIT_START_CALLBACK:        *const c_char = b"preeditStartCallback\0"       .as_ptr() as *const _;
    pub const PREEDIT_DONE_CALLBACK:         *const c_char = b"preeditDoneCallback\0"        .as_ptr() as *const _;
    pub const PREEDIT_DRAW_CALLBACK:         *const c_char = b"preeditDrawCallback\0"        .as_ptr() as *const _;
    pub const PREEDIT_CARET_CALLBACK:        *const c_char = b"preeditCaretCallback\0"       .as_ptr() as *const _;
    pub const PREEDIT_STATE_NOTIFY_CALLBACK: *const c_char = b"preeditStateNotifyCallback\0" .as_ptr() as *const _;
    pub const PREEDIT_ATTRIBUTES:            *const c_char = b"preeditAttributes\0"          .as_ptr() as *const _;
    pub const STATUS_START_CALLBACK:         *const c_char = b"statusStartCallback\0"        .as_ptr() as *const _;
    pub const STATUS_DONE_CALLBACK:          *const c_char = b"statusDoneCallback\0"         .as_ptr() as *const _;
    pub const STATUS_DRAW_CALLBACK:          *const c_char = b"statusDrawCallback\0"         .as_ptr() as *const _;
    pub const STATUS_ATTRIBUTES:             *const c_char = b"statusAttributes\0"           .as_ptr() as *const _;
    pub const AREA:                          *const c_char = b"area\0"                       .as_ptr() as *const _;
    pub const AREA_NEEDED:                   *const c_char = b"areaNeeded\0"                 .as_ptr() as *const _;
    pub const SPOT_LOCATION:                 *const c_char = b"spotLocation\0"               .as_ptr() as *const _;
    pub const COLORMAP:                      *const c_char = b"colorMap\0"                   .as_ptr() as *const _;
    pub const STD_COLORMAP:                  *const c_char = b"stdColorMap\0"                .as_ptr() as *const _;
    pub const FOREGROUND:                    *const c_char = b"foreground\0"                 .as_ptr() as *const _;
    pub const BACKGROUND:                    *const c_char = b"background\0"                 .as_ptr() as *const _;
    pub const BACKGROUND_PIXMAP:             *const c_char = b"backgroundPixmap\0"           .as_ptr() as *const _;
    pub const FONT_SET:                      *const c_char = b"fontSet\0"                    .as_ptr() as *const _;
    pub const LINE_SPACE:                    *const c_char = b"lineSpace\0"                  .as_ptr() as *const _;
    pub const CURSOR:                        *const c_char = b"cursor\0"                     .as_ptr() as *const _;
    pub const QUERY_IM_VALUES_LIST:          *const c_char = b"queryIMValuesList\0"          .as_ptr() as *const _;
    pub const QUERY_IC_VALUES_LIST:          *const c_char = b"queryICValuesList\0"          .as_ptr() as *const _;
    pub const VISIBLE_POSITION:              *const c_char = b"visiblePosition\0"            .as_ptr() as *const _;
    pub const R6_PREEDIT_CALLBACK:           *const c_char = b"r6PreeditCallback\0"          .as_ptr() as *const _;
    pub const STRING_CONVERSION_CALLBACK:    *const c_char = b"stringConversionCallback\0"   .as_ptr() as *const _;
    pub const STRING_CONVERSION:             *const c_char = b"stringConversion\0"           .as_ptr() as *const _;
    pub const RESET_STATE:                   *const c_char = b"resetState\0"                 .as_ptr() as *const _;
    pub const HOT_KEY:                       *const c_char = b"hotKey\0"                     .as_ptr() as *const _;
    pub const HOT_KEY_STATE:                 *const c_char = b"hotKeyState\0"                .as_ptr() as *const _;
    pub const PREEDIT_STATE:                 *const c_char = b"preeditState\0"               .as_ptr() as *const _;
    pub const SEPARATOROF_NESTED_LIST:       *const c_char = b"separatorofNestedList\0"      .as_ptr() as *const _;
}

pub mod prop_mode {
    use std::ffi::c_int;

    pub const REPLACE: c_int = 0;
    pub const PREPEND: c_int = 1;
    pub const APPEND: c_int = 2;
}

library! {
    [LibX11 <-> "X11"];

    pub fn XFree(data: *mut c_void) -> c_int;
    pub fn XFlush(display: *mut XDisplay);
    pub fn XInitThreads() -> Status;
    pub fn XrmInitialize();

    pub fn XSetErrorHandler(handler: Option<XErrorHandler>) -> Option<XErrorHandler>;

    pub fn XOpenDisplay(display_name: *const c_char) -> *mut XDisplay;
    pub fn XCloseDisplay(display: *mut XDisplay);
    pub fn XDefaultScreen(display: *mut XDisplay) -> c_int;
    pub fn XDefaultRootWindow(display: *mut XDisplay) -> XWindow;

    pub fn XCreateWindow(
        display: *mut XDisplay,
        parent: XWindow,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        depth: c_int,
        class: c_uint,
        visual: *mut Visual,
        value_mask: c_ulong,
        attributes: *mut XSetWindowAttributes,
    ) -> XWindow;

    pub fn XCreateSimpleWindow(
        display: *mut XDisplay,
        parent: XWindow,
        x: c_int,
        y: c_int,
        width: c_uint,
        height: c_uint,
        border_width: c_uint,
        border: c_ulong,
        background: c_ulong,
    ) -> XWindow;

    pub fn XGetAtomName(display: *mut XDisplay, atom: Atom) -> *const c_char;
    pub fn XInternAtom(
        display: *mut XDisplay,
        atom_name: *const c_char,
        only_if_exists: Bool,
    ) -> Atom;
    pub fn XSetWMProtocols(
        display: *mut XDisplay,
        w: XWindow,
        protocols: *const Atom,
        count: c_int,
    ) -> Status;
    pub fn XMaxRequestSize(display: *mut XDisplay) -> c_long;

    pub fn XGetWindowProperty(
        display: *mut XDisplay,
        win: XWindow,
        property: Atom,
        long_offset: c_long,
        long_length: c_long,
        delete: Bool,
        req_type: Atom,
        actual_type_return: *mut Atom,
        actual_format_return: *mut c_int,
        nitems_return: *mut c_ulong,
        bytes_after_return: *mut c_ulong,
        prop_return: *mut *mut c_void,
    ) -> c_int;
    pub fn XChangeProperty(
        display: *mut XDisplay,
        win: XWindow,
        property: Atom,
        ty: Atom,
        format: c_int,
        mode: c_int,
        data: *const c_void,
        nelements: c_int
    );
    pub fn XDeleteProperty(display: *mut XDisplay, win: XWindow, property: Atom);

    pub fn XMapWindow(display: *mut XDisplay, window: XWindow);
    pub fn XUnmapWindow(display: *mut XDisplay, window: XWindow);

    pub fn Xutf8LookupString(
        ic: XIC,
        event: *const XKeyPressedEvent,
        buffer_return: *mut c_char,
        bytes_buffer: c_int,
        keysym_return: *mut KeySym,
        status_return: *mut Status,
    ) -> c_int;

    pub fn XSetLocaleModifiers(modifier_list: *const c_char) -> *mut c_char;

    pub fn XStoreName(display: *mut XDisplay, window: XWindow, window_name: *const c_char);

    // Selections

    pub fn XGetSelectionOwner(display: *mut XDisplay, selection: Atom) -> XWindow;
    pub fn XSetSelectionOwner(
        display: *mut XDisplay,
        selection: Atom,
        owner: XWindow,
        time: Time
    ) -> c_int;
    pub fn XConvertSelection(
        display: *mut XDisplay,
        selection: Atom,
        target: Atom,
        property: Atom,
        requestor: XWindow,
        time: Time
    ) -> c_int;

    // Event Handling

    pub fn XPending(display: *mut XDisplay) -> c_int;
    pub fn XNextEvent(display: *mut XDisplay, event: *mut XEvent) -> c_int;
    pub fn XSendEvent(
        display: *mut XDisplay,
        window: XWindow,
        propagate: c_int,
        event_mask: c_long,
        event_send: *mut XEvent,
    ) -> c_int;

    // XKB

    pub fn XkbSetDetectableAutoRepeat(
        display: *mut XDisplay,
        detectable: Bool,
        supported: *mut Bool,
    ) -> Bool;

    // IME

    pub fn XOpenIM(
        display: *mut XDisplay,
        db: XrmDatabase,
        res_name: *const c_char,
        res_class: *const c_char,
    ) -> XIM;
    pub fn XCreateIC(im: XIM, args: ...) -> XIC;
    pub fn XSetICFocus(ic: XIC);
    pub fn XUnsetICFocus(ic: XIC);
    pub fn XSetICValues(ic: XIC, args: ...) -> *const c_char;
    pub fn XCloseIM(im: XIM) -> Status;

    pub fn XSelectInput(display: *mut XDisplay, window: XWindow, event_mask: c_long);
    pub fn XFilterEvent(event: *mut XEvent, window: XWindow) -> Bool;
    pub fn XVaCreateNestedList(_unused: c_int, args: ...) -> XVaNestedList;
}
