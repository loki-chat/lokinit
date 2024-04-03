use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, WlInterface, NULLPTR};

#[repr(C)]
pub struct WlCallback([u8; 0]);

#[repr(C)]
pub struct WlCallbackListener {
    pub done: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_callback: *mut WlCallback, callback_data: u32),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_callback_add_listener(
        &self,
        wl_callback: *mut WlCallback,
        listener: *const WlCallbackListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_callback as _, listener as _, data)
    }

    pub unsafe fn wl_callback_set_user_data(
        &self,
        wl_callback: *mut WlCallback,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_callback as _, user_data)
    }

    pub unsafe fn wl_callback_get_user_data(&self, wl_callback: *mut WlCallback) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_callback as _)
    }

    pub unsafe fn wl_callback_get_version(&self, wl_callback: *mut WlCallback) -> u32 {
        (self.wl_proxy_get_version)(wl_callback as _)
    }

    pub unsafe fn wl_callback_destroy(&self, wl_callback: *mut WlCallback) {
        (self.wl_proxy_destroy)(wl_callback as _)
    }
}
