use std::ffi::c_void;

use crate::wayland::{LibWaylandClient, WL_MARSHAL_FLAG_DESTROY};

use super::wl_surface::WlSurface;

#[repr(C)]
pub struct WlSubsurface([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlSubsurfaceError {
    /// wl_surface is not a sibling or the parent
    BadSurface = 0,
}

pub const WL_SUBSURFACE_DESTROY: u32 = 0;
pub const WL_SUBSURFACE_SET_POSITION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE: u32 = 2;
pub const WL_SUBSURFACE_PLACE_BELOW: u32 = 3;
pub const WL_SUBSURFACE_SET_SYNC: u32 = 4;
pub const WL_SUBSURFACE_SET_DESYNC: u32 = 5;
pub const WL_SUBSURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_POSITION_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_BELOW_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_DESYNC_SINCE_VERSION: u32 = 1;

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_subsurface_set_user_data(
        &self,
        wl_subsurface: *mut WlSubsurface,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_subsurface as _, user_data)
    }

    pub unsafe fn wl_subsurface_get_user_data(
        &self,
        wl_subsurface: *mut WlSubsurface,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_subsurface as _)
    }

    pub unsafe fn wl_subsurface_get_version(&self, wl_subsurface: *mut WlSubsurface) -> u32 {
        (self.wl_proxy_get_version)(wl_subsurface as _)
    }

    pub unsafe fn wl_subsurface_destroy(&self, wl_subsurface: *mut WlSubsurface) {
        (self.wl_proxy_marshal_flags)(
            wl_subsurface as _,
            WL_SUBSURFACE_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subsurface as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }

    pub unsafe fn wl_subsurface_set_position(
        &self,
        wl_subsurface: *mut WlSubsurface,
        x: i32,
        y: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_subsurface as _,
            WL_SUBSURFACE_SET_POSITION,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subsurface as _),
            0,
            x,
            y,
        );
    }

    pub unsafe fn wl_subsurface_place_above(
        &self,
        wl_subsurface: *mut WlSubsurface,
        sibling: *mut WlSurface,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_subsurface as _,
            WL_SUBSURFACE_PLACE_ABOVE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subsurface as _),
            0,
            sibling,
        );
    }

    pub unsafe fn wl_subsurface_place_below(
        &self,
        wl_subsurface: *mut WlSubsurface,
        sibling: *mut WlSurface,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_subsurface as _,
            WL_SUBSURFACE_PLACE_BELOW,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subsurface as _),
            0,
            sibling,
        );
    }

    pub unsafe fn wl_subsurface_set_sync(&self, wl_subsurface: *mut WlSubsurface) {
        (self.wl_proxy_marshal_flags)(
            wl_subsurface as _,
            WL_SUBSURFACE_SET_SYNC,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subsurface as _),
            0,
        );
    }

    pub unsafe fn wl_subsurface_set_desync(&self, wl_subsurface: *mut WlSubsurface) {
        (self.wl_proxy_marshal_flags)(
            wl_subsurface as _,
            WL_SUBSURFACE_SET_DESYNC,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subsurface as _),
            0,
        );
    }
}
