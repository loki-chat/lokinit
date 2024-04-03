use std::ffi::c_void;

use crate::wayland::{LibWaylandClient, WL_MARSHAL_FLAG_DESTROY};

#[repr(C)]
pub struct WlRegion([u8; 0]);

pub const WL_REGION_DESTROY: u32 = 0;
pub const WL_REGION_ADD: u32 = 1;
pub const WL_REGION_SUBTRACT: u32 = 2;
pub const WL_REGION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_REGION_ADD_SINCE_VERSION: u32 = 1;
pub const WL_REGION_SUBTRACT_SINCE_VERSION: u32 = 1;

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_region_set_user_data(&self, wl_region: *mut WlRegion, user_data: *mut c_void) {
        (self.wl_proxy_set_user_data)(wl_region as _, user_data)
    }

    pub unsafe fn wl_region_get_user_data(&self, wl_region: *mut WlRegion) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_region as _)
    }

    pub unsafe fn wl_region_get_version(&self, wl_region: *mut WlRegion) -> u32 {
        (self.wl_proxy_get_version)(wl_region as _)
    }

    pub unsafe fn wl_region_destroy(&self, wl_region: *mut WlRegion) {
        (self.wl_proxy_marshal_flags)(
            wl_region as _,
            WL_REGION_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_region as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }

    pub unsafe fn wl_region_add(
        &self,
        wl_region: *mut WlRegion,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_region as _,
            WL_REGION_ADD,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_region as _),
            0,
            x,
            y,
            width,
            height,
        );
    }

    pub unsafe fn wl_region_subtract(
        &self,
        wl_region: *mut WlRegion,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_region as _,
            WL_REGION_SUBTRACT,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_region as _),
            0,
            x,
            y,
            width,
            height,
        );
    }
}
