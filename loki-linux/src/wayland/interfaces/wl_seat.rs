use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, NULLPTR, WL_MARSHAL_FLAG_DESTROY};

use super::wl_keyboard::WlKeyboard;
use super::wl_pointer::WlPointer;
use super::wl_touch::WlTouch;

#[repr(C)]
pub struct WlSeat([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlSeatCapability {
    /// the seat has pointer devices
    Pointer = 1,
    /// the seat has one or more keyboards
    Keyboard = 2,
    /// the seat has touch devices
    Touch = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlSeatError {
    /// get_pointer, get_keyboard or get_touch called on seat without the matching capability
    MissingCapability = 0,
}

pub const WL_SEAT_GET_POINTER: u32 = 0;
pub const WL_SEAT_GET_KEYBOARD: u32 = 1;
pub const WL_SEAT_GET_TOUCH: u32 = 2;
pub const WL_SEAT_RELEASE: u32 = 3;
pub const WL_SEAT_CAPABILITIES_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_NAME_SINCE_VERSION: u32 = 2;
pub const WL_SEAT_GET_POINTER_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_KEYBOARD_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_TOUCH_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_RELEASE_SINCE_VERSION: u32 = 5;

pub struct WlSeatListener {
    pub capabilities:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_seat: *mut WlSeat, capabilities: u32)>,
    pub name:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_seat: *mut WlSeat, name: *const c_char)>,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_seat_add_listener(
        &self,
        wl_seat: *mut WlSeat,
        listener: *const WlSeatListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_seat as _, listener as _, data)
    }

    pub unsafe fn wl_seat_set_user_data(&self, wl_seat: *mut WlSeat, user_data: *mut c_void) {
        (self.wl_proxy_set_user_data)(wl_seat as _, user_data)
    }

    pub unsafe fn wl_seat_get_user_data(&self, wl_seat: *mut WlSeat) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_seat as _)
    }

    pub unsafe fn wl_seat_get_version(&self, wl_seat: *mut WlSeat) -> u32 {
        (self.wl_proxy_get_version)(wl_seat as _)
    }

    pub unsafe fn wl_seat_destroy(&self, wl_seat: *mut WlSeat) {
        (self.wl_proxy_destroy)(wl_seat as _)
    }

    pub unsafe fn wl_seat_get_pointer(&self, wl_seat: *mut WlSeat) -> *mut WlPointer {
        (self.wl_proxy_marshal_flags)(
            wl_seat as _,
            WL_SEAT_GET_POINTER,
            self.wl_pointer_interface,
            (self.wl_proxy_get_version)(wl_seat as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_seat_get_keyboard(&self, wl_seat: *mut WlSeat) -> *mut WlKeyboard {
        (self.wl_proxy_marshal_flags)(
            wl_seat as _,
            WL_SEAT_GET_KEYBOARD,
            self.wl_keyboard_interface,
            (self.wl_proxy_get_version)(wl_seat as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_seat_get_touch(&self, wl_seat: *mut WlSeat) -> *mut WlTouch {
        (self.wl_proxy_marshal_flags)(
            wl_seat as _,
            WL_SEAT_GET_TOUCH,
            self.wl_touch_interface,
            (self.wl_proxy_get_version)(wl_seat as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_seat_release(&self, wl_seat: *mut WlSeat) {
        (self.wl_proxy_marshal_flags)(
            wl_seat as _,
            WL_SEAT_RELEASE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_seat as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}
