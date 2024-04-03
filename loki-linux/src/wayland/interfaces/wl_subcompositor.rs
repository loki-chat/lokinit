use std::ffi::c_void;

use crate::wayland::{LibWaylandClient, NULLPTR, WL_MARSHAL_FLAG_DESTROY};

use super::wl_subsurface::WlSubsurface;
use super::wl_surface::WlSurface;

#[repr(C)]
pub struct WlSubcompositor([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlSubcompositorError {
    /// the to-be sub-surface is invalid
    BadSurface = 0,
    /// the to-be sub-surface parent is invalid
    BadParent = 1,
}

pub const WL_SUBCOMPOSITOR_DESTROY: u32 = 0;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE: u32 = 1;
pub const WL_SUBCOMPOSITOR_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE_SINCE_VERSION: u32 = 1;

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_subcompositor_set_user_data(
        &self,
        wl_subcompositor: *mut WlSubcompositor,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_subcompositor as _, user_data)
    }

    pub unsafe fn wl_subcompositor_get_user_data(
        &self,
        wl_subcompositor: *mut WlSubcompositor,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_subcompositor as _)
    }

    pub unsafe fn wl_subcompositor_get_version(
        &self,
        wl_subcompositor: *mut WlSubcompositor,
    ) -> u32 {
        (self.wl_proxy_get_version)(wl_subcompositor as _)
    }

    pub unsafe fn wl_subcompositor_destroy(&self, wl_subcompositor: *mut WlSubcompositor) {
        (self.wl_proxy_marshal_flags)(
            wl_subcompositor as _,
            WL_SUBCOMPOSITOR_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_subcompositor as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }

    pub unsafe fn wl_subcompositor_get_subsurface(
        &self,
        wl_subcompositor: *mut WlSubcompositor,
        surface: *mut WlSurface,
        parent: *mut WlSurface,
    ) -> *mut WlSubsurface {
        (self.wl_proxy_marshal_flags)(
            wl_subcompositor as _,
            WL_SUBCOMPOSITOR_GET_SUBSURFACE,
            self.wl_subsurface_interface,
            (self.wl_proxy_get_version)(wl_subcompositor as _),
            0,
            NULLPTR,
            surface,
            parent,
        ) as _
    }
}
