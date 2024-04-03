use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, WL_MARSHAL_FLAG_DESTROY};

pub const WL_BUFFER_DESTROY: u32 = 0;
pub const WL_BUFFER_RELEASE_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_DESTROY_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlBuffer([u8; 0]);

#[repr(C)]
pub struct WlBufferListener {
    done: Option<unsafe extern "C" fn(data: *mut c_void, wl_buffer: *mut WlBuffer)>,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_buffer_add_listener(
        &self,
        wl_buffer: *mut WlBuffer,
        listener: *const WlBufferListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_buffer as _, listener as _, data)
    }

    pub unsafe fn wl_buffer_set_user_data(&self, wl_buffer: *mut WlBuffer, user_data: *mut c_void) {
        (self.wl_proxy_set_user_data)(wl_buffer as _, user_data)
    }

    pub unsafe fn wl_buffer_get_user_data(&self, wl_buffer: *mut WlBuffer) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_buffer as _)
    }

    pub unsafe fn wl_buffer_get_version(&self, wl_buffer: *mut WlBuffer) -> u32 {
        (self.wl_proxy_get_version)(wl_buffer as _)
    }

    pub unsafe fn wl_buffer_destroy(&self, wl_buffer: *mut WlBuffer) {
        (self.wl_proxy_marshal_flags)(
            wl_buffer as _,
            WL_BUFFER_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_buffer as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}
