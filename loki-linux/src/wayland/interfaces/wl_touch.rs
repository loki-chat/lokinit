use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, WlFixed, WL_MARSHAL_FLAG_DESTROY};

use super::wl_surface::WlSurface;

#[repr(C)]
pub struct WlTouch([u8; 0]);

pub const WL_TOUCH_RELEASE: u32 = 0;
pub const WL_TOUCH_DOWN_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_UP_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_CANCEL_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_SHAPE_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_ORIENTATION_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_RELEASE_SINCE_VERSION: u32 = 3;

pub struct WlTouchListener {
    pub down: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_touch: *mut WlTouch,
            serial: u32,
            time: u32,
            surface: *mut WlSurface,
            id: i32,
            x: WlFixed,
            y: WlFixed,
        ),
    >,

    pub up: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_touch: *mut WlTouch,
            serial: u32,
            time: u32,
            id: i32,
        ),
    >,

    pub motion: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_touch: *mut WlTouch,
            time: u32,
            id: i32,
            x: WlFixed,
            y: WlFixed,
        ),
    >,

    pub frame: Option<unsafe extern "C" fn(data: *mut c_void, wl_touch: *mut WlTouch)>,

    pub cancel: Option<unsafe extern "C" fn(data: *mut c_void, wl_touch: *mut WlTouch)>,

    pub shape: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_touch: *mut WlTouch,
            id: i32,
            major: WlFixed,
            minor: WlFixed,
        ),
    >,

    pub orientation: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_touch: *mut WlTouch,
            id: i32,
            orientation: WlFixed,
        ),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_touch_add_listener(
        &self,
        wl_touch: *mut WlTouch,
        listener: *const WlTouchListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_touch as _, listener as _, data)
    }

    pub unsafe fn wl_touch_set_user_data(
        &self,
        wl_touch: *mut WlTouch,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_touch as _, user_data)
    }

    pub unsafe fn wl_touch_get_user_data(&self, wl_touch: *mut WlTouch) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_touch as _)
    }

    pub unsafe fn wl_touch_get_version(&self, wl_touch: *mut WlTouch) -> u32 {
        (self.wl_proxy_get_version)(wl_touch as _)
    }

    pub unsafe fn wl_touch_destroy(&self, wl_touch: *mut WlTouch) {
        (self.wl_proxy_destroy)(wl_touch as _)
    }

    pub unsafe fn wl_touch_release(&self, wl_touch: *mut WlTouch) {
        (self.wl_proxy_marshal_flags)(
            wl_touch as _,
            WL_TOUCH_RELEASE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_touch as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}
