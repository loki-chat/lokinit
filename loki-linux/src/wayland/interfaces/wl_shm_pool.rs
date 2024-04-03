use std::ffi::c_void;

use crate::wayland::{LibWaylandClient, NULLPTR, WL_MARSHAL_FLAG_DESTROY};

use super::wl_buffer::WlBuffer;

pub const WL_SHM_POOL_CREATE_BUFFER: u32 = 0;
pub const WL_SHM_POOL_DESTROY: u32 = 1;
pub const WL_SHM_POOL_RESIZE: u32 = 2;
pub const WL_SHM_POOL_CREATE_BUFFER_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_RESIZE_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlShmPool([u8; 0]);

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_shm_pool_set_user_data(
        &self,
        wl_shm_pool: *mut WlShmPool,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_shm_pool as _, user_data)
    }

    pub unsafe fn wl_shm_pool_get_user_data(&self, wl_shm_pool: *mut WlShmPool) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_shm_pool as _)
    }

    pub unsafe fn wl_shm_pool_get_version(&self, wl_shm_pool: *mut WlShmPool) -> u32 {
        (self.wl_proxy_get_version)(wl_shm_pool as _)
    }

    pub unsafe fn wl_shm_pool_create_buffer(
        &self,
        wl_shm_pool: *mut WlShmPool,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    ) -> *mut WlBuffer {
        (self.wl_proxy_marshal_flags)(
            wl_shm_pool as _,
            WL_SHM_POOL_CREATE_BUFFER,
            self.wl_buffer_interface,
            (self.wl_proxy_get_version)(wl_shm_pool as _),
            0,
            NULLPTR,
            offset,
            width,
            height,
            stride,
            format,
        ) as _
    }

    pub unsafe fn wl_shm_pool_destroy(&self, wl_shm_pool: *mut WlShmPool) {
        (self.wl_proxy_marshal_flags)(wl_shm_pool as _, WL_SHM_POOL_DESTROY, std::ptr::null(), (self.wl_proxy_get_version)(wl_shm_pool as _), WL_MARSHAL_FLAG_DESTROY);
    }

    pub unsafe fn wl_shm_pool_resize(&self, wl_shm_pool: *mut WlShmPool, size: i32) {
        (self.wl_proxy_marshal_flags)(
            wl_shm_pool as _,
            WL_SHM_POOL_RESIZE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shm_pool as _),
            0,
            size,
        );
    }
}
