use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, WlFixed, WL_MARSHAL_FLAG_DESTROY};

use super::wl_surface::WlSurface;

#[repr(C)]
pub struct WlPointer([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlPointerError {
    /// given wl_surface has another role
    Role = 0,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlPointerButtonState {
    /// the button is not pressed
    Released = 0,
    /// the button is pressed
    Pressed = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlPointerAxis {
    /// vertical axis
    VerticalScroll = 0,
    /// horizontal axis
    HorizontalScroll = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlPointerAxisSource {
    /// a physical wheel rotation
    Wheel = 0,
    /// finger on a touch surface
    Finger = 1,
    /// continuous coordinate space
    Continuous = 2,
    /// a physical wheel tilt
    WheelTilt = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlPointerAxisRelativeDirection {
    /// physical motion matches axis direction
    Identical = 0,
    /// physical motion is the inverse of the axis direction
    Inverted = 1,
}

pub const WL_POINTER_SET_CURSOR: u32 = 0;
pub const WL_POINTER_RELEASE: u32 = 1;
pub const WL_POINTER_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_BUTTON_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_AXIS_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_FRAME_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_STOP_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_DISCRETE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_VALUE120_SINCE_VERSION: u32 = 8;
pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_SINCE_VERSION: u32 = 9;
pub const WL_POINTER_SET_CURSOR_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_RELEASE_SINCE_VERSION: u32 = 3;

pub struct WlPointerListener {
    pub enter: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            serial: u32,
            surface: *mut WlSurface,
            surface_x: WlFixed,
            surface_y: WlFixed,
        ),
    >,

    pub leave: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            serial: u32,
            surface: *mut WlSurface,
        ),
    >,

    pub motion: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            time: u32,
            surface_x: WlFixed,
            surface_y: WlFixed,
        ),
    >,

    pub button: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            serial: u32,
            time: u32,
            button: u32,
            state: u32,
        ),
    >,

    pub axis: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            time: u32,
            axis: u32,
            value: WlFixed,
        ),
    >,

    pub frame: Option<unsafe extern "C" fn(data: *mut c_void, wl_pointer: *mut WlPointer)>,

    pub axis_source: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_pointer: *mut WlPointer, axis_source: u32),
    >,

    pub axis_stop: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_pointer: *mut WlPointer, time: u32, axis: u32),
    >,

    pub axis_discrete: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            axis: u32,
            discrete: i32,
        ),
    >,

    pub axis_value120: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            axis: u32,
            value120: i32,
        ),
    >,

    pub axis_relative_direction: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_pointer: *mut WlPointer,
            axis: u32,
            direction: u32,
        ),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_pointer_add_listener(
        &self,
        wl_pointer: *mut WlPointer,
        listener: *const WlPointerListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_pointer as _, listener as _, data)
    }

    pub unsafe fn wl_pointer_set_user_data(
        &self,
        wl_pointer: *mut WlPointer,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_pointer as _, user_data)
    }

    pub unsafe fn wl_pointer_get_user_data(&self, wl_pointer: *mut WlPointer) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_pointer as _)
    }

    pub unsafe fn wl_pointer_get_version(&self, wl_pointer: *mut WlPointer) -> u32 {
        (self.wl_proxy_get_version)(wl_pointer as _)
    }

    pub unsafe fn wl_pointer_destroy(&self, wl_pointer: *mut WlPointer) {
        (self.wl_proxy_destroy)(wl_pointer as _)
    }

    pub unsafe fn wl_pointer_set_cursor(
        &self,
        wl_pointer: *mut WlPointer,
        serial: u32,
        surface: *mut WlSurface,
        hotspot_x: i32,
        hotspot_y: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_pointer as _,
            WL_POINTER_SET_CURSOR,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_pointer as _),
            0,
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
    }

    pub unsafe fn wl_pointer_release(&self, wl_pointer: *mut WlPointer) {
        (self.wl_proxy_marshal_flags)(
            wl_pointer as _,
            WL_POINTER_RELEASE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_pointer as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}
