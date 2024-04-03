use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, NULLPTR};

use super::wl_region::WlRegion;
use super::wl_surface::WlSurface;

pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_COMPOSITOR_CREATE_REGION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_REGION_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlCompositor([u8; 0]);

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_compositor_set_user_data(
        &self,
        wl_compositor: *mut WlCompositor,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_compositor as _, user_data)
    }

    pub unsafe fn wl_compositor_get_user_data(
        &self,
        wl_compositor: *mut WlCompositor,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_compositor as _)
    }

    pub unsafe fn wl_compositor_get_version(&self, wl_compositor: *mut WlCompositor) -> u32 {
        (self.wl_proxy_get_version)(wl_compositor as _)
    }

    pub unsafe fn wl_compositor_destroy(&self, wl_compositor: *mut WlCompositor) {
        (self.wl_proxy_destroy)(wl_compositor as _)
    }

    pub unsafe fn wl_compositor_create_surface(
        &self,
        wl_compositor: *mut WlCompositor,
    ) -> *mut WlSurface {
        (self.wl_proxy_marshal_flags)(
            wl_compositor as _,
            WL_COMPOSITOR_CREATE_SURFACE,
            self.wl_surface_interface,
            (self.wl_proxy_get_version)(wl_compositor as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_compositor_create_region(
        &self,
        wl_compositor: *mut WlCompositor,
    ) -> *mut WlRegion {
        (self.wl_proxy_marshal_flags)(
            wl_compositor as _,
            WL_COMPOSITOR_CREATE_REGION,
            self.wl_region_interface,
            (self.wl_proxy_get_version)(wl_compositor as _),
            0,
            NULLPTR,
        ) as _
    }
}
