use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, WlRegistry, NULLPTR};

use super::wl_callback::WlCallback;

pub const WL_DISPLAY_SYNC: u32 = 0;
pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_DISPLAY_ERROR_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_DELETE_ID_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_GET_REGISTRY_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlDisplay([u8; 0]);

#[repr(C)]
pub struct WlDisplayListener {
    pub error: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_display: *mut WlDisplay,
            object_id: *mut c_void,
            code: u32,
            message: *const c_char,
        ),
    >,
    pub delete_id: Option<unsafe extern "C" fn(data: *mut c_void, wl_display: *mut WlDisplay, id: u32)>,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_display_add_listener(
        &self,
        wl_display: *mut WlDisplay,
        listener: *const WlDisplayListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_display as _, listener as _, data)
    }

    pub unsafe fn wl_display_set_user_data(
        &self,
        wl_display: *mut WlDisplay,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_display as _, user_data)
    }

    pub unsafe fn wl_display_get_user_data(&self, wl_display: *mut WlDisplay) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_display as _)
    }

    pub unsafe fn wl_display_get_version(&self, wl_display: *mut WlDisplay) -> u32 {
        (self.wl_proxy_get_version)(wl_display as _)
    }

    pub unsafe fn wl_display_sync(&self, wl_display: *mut WlDisplay) -> *mut WlCallback {
        (self.wl_proxy_marshal_flags)(
            wl_display as _,
            WL_DISPLAY_SYNC,
            self.wl_callback_interface,
            (self.wl_proxy_get_version)(wl_display as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_display_get_registry(&self, wl_display: *mut WlDisplay) -> *mut WlRegistry {
        (self.wl_proxy_marshal_flags)(
            wl_display as _,
            WL_DISPLAY_GET_REGISTRY,
            self.wl_registry_interface,
            (self.wl_proxy_get_version)(wl_display as _),
            0,
            NULLPTR,
        ) as _
    }
}
