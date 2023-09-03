use std::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong};
use std::fmt::Debug;

use super::{Atom, Bool, Colormap, Drawable, Time, XDisplay, XWindow, XID};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XAnyEvent {
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XButtonEvent {
    /// ButtonPress or ButtonRelease
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// "event" window it is reported relative to
    pub window: XWindow,
    /// root window that the event occurred on
    pub root: XWindow,
    /// child window
    pub subwindow: XWindow,
    /// milliseconds
    pub time: Time,
    /// pointer x coordinate in event window
    pub x: c_int,
    /// pointer y coordinate in event window
    pub y: c_int,
    /// x coordinate relative to root
    pub x_root: c_int,
    /// y coordinate relative to root
    pub y_root: c_int,
    /// key or button mask
    pub state: c_int,
    /// detail
    pub button: c_int,
    /// same screen flag
    pub same_screen: Bool,
}
pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XKeyEvent {
    /// KeyPress or KeyRelease
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// "event" window it is reported relative to
    pub window: XWindow,
    /// root window that the event occurred on
    pub root: XWindow,
    /// child window
    pub subwindow: XWindow,
    /// milliseconds
    pub time: Time,
    /// pointer x coordinate in event window
    pub x: c_int,
    /// pointer y coordinate in event window
    pub y: c_int,
    /// x coordinate relative to root
    pub x_root: c_int,
    /// y coordinate relative to root
    pub y_root: c_int,
    /// key or button mask
    pub state: c_uint,
    /// detail
    pub keycode: c_uint,
    /// same screen flag
    pub same_screen: Bool,
}
pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XMotionEvent {
    /// KeyPress or KeyRelease
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// "event" window it is reported relative to
    pub window: XWindow,
    /// root window that the event occurred on
    pub root: XWindow,
    /// child window
    pub subwindow: XWindow,
    /// milliseconds
    pub time: Time,
    /// pointer x coordinate in event window
    pub x: c_int,
    /// pointer y coordinate in event window
    pub y: c_int,
    /// x coordinate relative to root
    pub x_root: c_int,
    /// y coordinate relative to root
    pub y_root: c_int,
    /// key or button mask
    pub state: c_uint,
    /// detail
    pub is_hint: bool,
    /// same screen flag
    pub same_screen: Bool,
}
pub type XPointerMovedEvent = XMotionEvent;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XCrossingEvent {
    /// EnterNotify or LeaveNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// "event" window it is reported relative to
    pub window: XWindow,
    /// root window that the event occurred on
    pub root: XWindow,
    /// child window
    pub subwindow: XWindow,
    /// milliseconds
    pub time: Time,
    /// pointer x coordinate in event window
    pub x: c_int,
    /// pointer y coordinate in event window
    pub y: c_int,
    /// x coordinate relative to root
    pub x_root: c_int,
    /// y coordinate relative to root
    pub y_root: c_int,
    /// NotifyNormal, NotifyGrab, NotifyUngrab
    pub mode: c_int,
    /// NotifyAncestor, NotifyVirtual, NotifyInferior,
    /// NotifyNonlinear, NotifyNonlinearVirtual
    pub detail: c_int,
    /// same screen flag
    pub same_screen: Bool,
    /// boolean focus
    pub focus: Bool,
    /// key or button mask
    pub state: c_uint,
}
pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XFocusChangeEvent {
    /// EnterNotify or LeaveNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// "event" window it is reported relative to
    pub window: XWindow,
    /// NotifyNormal, NotifyGrab, NotifyUngrab
    pub mode: c_int,
    /// NotifyAncestor, NotifyVirtual, NotifyInferior,
    /// NotifyNonlinear, NotifyNonlinearVirtual
    pub detail: c_int,
}
pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XExposeEvent {
    /// Expose
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// "event" window it is reported relative to
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    /// if nonzero, at least this many more
    pub count: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XGraphicsExposeEvent {
    /// Expose
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub drawable: Drawable,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    /// if nonzero, at least this many more
    pub count: c_int,
    /// core is CopyArea or CopyPlane
    pub major_code: c_int,
    /// not defined in the core
    pub minor_code: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XNoExposeEvent {
    /// NoExpose
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub drawable: Drawable,
    /// core is CopyArea or CopyPlane
    pub major_code: c_int,
    /// not defined in the core
    pub minor_code: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XVisibilityEvent {
    /// VisibiltyNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub state: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XCreateWindowEvent {
    /// CreateNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// parent of the window
    pub parent: XWindow,
    /// window id of window created
    pub window: XWindow,
    /// x coordinate of window location
    pub x: c_int,
    /// y coordinate of window location
    pub y: c_int,
    /// width of window
    pub width: c_int,
    /// height of window
    pub height: c_int,
    /// border width
    pub border_width: c_int,
    /// creation should be overridden
    pub override_direct: Bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XDestroyWindowEvent {
    /// DestroyNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XUnmapEvent {
    /// UnmapNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub from_configure: Bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XMapEvent {
    /// UnmapNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    /// boolean, is override set...
    pub override_redirect: Bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XMapRequestEvent {
    /// CreateNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub parent: XWindow,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XReparentEvent {
    /// CreateNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub parent: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub override_direct: Bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XConfigureEvent {
    /// ConfigureNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: XWindow,
    pub override_direct: Bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XGravityEvent {
    /// GravityNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XResizeRequestEvent {
    /// ResizeRequest
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    pub width: c_int,
    pub height: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XConfigureRequestEvent {
    /// ConfigureRequest
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub parent: XWindow,
    pub window: XWindow,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: XWindow,
    /// Above, Below, TopIf, BottomIf, Opposite
    pub detail: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XCirculateEvent {
    /// CirculateNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    /// PlaceOnTop, PlaceOnBottom
    pub place: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XCirculateRequestEvent {
    /// CirculateRequest
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub event: XWindow,
    pub window: XWindow,
    /// PlaceOnTop, PlaceOnBottom
    pub place: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XPropertyEvent {
    /// PropertyNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub atom: Atom,
    pub time: Time,
    /// PropertyNewValue or PropertyDelete
    pub state: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XSelectionClearEvent {
    /// SelectionClear
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub selection: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XSelectionRequestEvent {
    /// SelectionRequest
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub owner: XWindow,
    pub requestor: XWindow,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XSelectionEvent {
    /// SelectionNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub requestor: XWindow,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XColormapEvent {
    /// ColormapNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    /// colormap or None
    pub colormap: Colormap,
    pub new: Bool,
    /// ColormapInstalled, ColormapUninstalled
    pub state: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union XClientMessageData {
    pub b: [c_char; 20],
    pub s: [c_short; 10],
    /// This is C being extremely stupid by using `long` intead of `int` for 32-bit values, so now this is 64 bit apparently...
    pub l: [c_long; 5],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XClientMessageEvent {
    /// ClientMessage
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub message_type: Atom,
    pub format: c_int,
    pub data: XClientMessageData,
}

impl std::fmt::Debug for XClientMessageEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("XClientMessageEvent");

        ds.field("type_id", &self.type_id)
            .field("serial", &self.serial)
            .field("send_event", &self.send_event)
            .field("display", &self.display)
            .field("window", &self.window)
            .field("message_type", &self.message_type)
            .field("format", &self.format);

        match self.format {
            8 => {
                ds.field("data", unsafe { &self.data.b });
            }
            16 => {
                ds.field("data", unsafe { &self.data.s });
            }
            32 => {
                ds.field("data", unsafe { &self.data.l });
            }
            _ => (),
        }

        ds.finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XMappingEvent {
    /// MappingNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    /// unused (???)
    pub window: XWindow,
    /// one of MappingModifier, MappingKeyboard, MappingPointer
    pub request: c_int,
    /// first keycode
    pub first_keycode: c_int,
    /// defines range of change w. first_keycode
    pub count: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XErrorEvent {
    /// KeymapNotify
    pub type_id: c_int,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub resourceid: XID,
    pub serial: c_ulong,
    pub error_code: u8,
    pub request_code: u8,
    pub minor_code: u8,
}

/// generated on EnterWindow and FocusIn when KeymapState selected
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XKeymapEvent {
    /// KeymapNotify
    pub type_id: c_int,
    /// \# of last request processed by server
    pub serial: c_ulong,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub display: *mut XDisplay,
    pub window: XWindow,
    pub key_vector: [i8; 32],
}

/// Event names. Used in "type" field in XEvent structures.  Not to be
/// confused with event masks. They start from 2 because 0 and 1
/// are reserved in the protocol for errors and replies.
pub mod et {
    use std::ffi::c_int;

    pub const KEY_PRESS: c_int = 2;
    pub const KEY_RELEASE: c_int = 3;
    pub const BUTTON_PRESS: c_int = 4;
    pub const BUTTON_RELEASE: c_int = 5;
    pub const MOTION_NOTIFY: c_int = 6;
    pub const ENTER_NOTIFY: c_int = 7;
    pub const LEAVE_NOTIFY: c_int = 8;
    pub const FOCUS_IN: c_int = 9;
    pub const FOCUS_OUT: c_int = 10;
    pub const KEYMAP_NOTIFY: c_int = 11;
    pub const EXPOSE: c_int = 12;
    pub const GRAPHICS_EXPOSE: c_int = 13;
    pub const NO_EXPOSE: c_int = 14;
    pub const VISIBILITY_NOTIFY: c_int = 15;
    pub const CREATE_NOTIFY: c_int = 16;
    pub const DESTROY_NOTIFY: c_int = 17;
    pub const UNMAP_NOTIFY: c_int = 18;
    pub const MAP_NOTIFY: c_int = 19;
    pub const MAP_REQUEST: c_int = 20;
    pub const REPARENT_NOTIFY: c_int = 21;
    pub const CONFIGURE_NOTIFY: c_int = 22;
    pub const CONFIGURE_REQUEST: c_int = 23;
    pub const GRAVITY_NOTIFY: c_int = 24;
    pub const RESIZE_REQUEST: c_int = 25;
    pub const CIRCULATE_NOTIFY: c_int = 26;
    pub const CIRCULATE_REQUEST: c_int = 27;
    pub const PROPERTY_NOTIFY: c_int = 28;
    pub const SELECTION_CLEAR: c_int = 29;
    pub const SELECTION_REQUEST: c_int = 30;
    pub const SELECTION_NOTIFY: c_int = 31;
    pub const COLORMAP_NOTIFY: c_int = 32;
    pub const CLIENT_MESSAGE: c_int = 33;
    pub const MAPPING_NOTIFY: c_int = 34;
    pub const GENERIC_EVENT: c_int = 35;

    /// must be bigger than any event #
    pub const LAST_EVENT: c_int = 36;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union XEvent {
    pub type_id: c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pad: [c_long; 24],
}
