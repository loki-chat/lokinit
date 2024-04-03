use std::ffi::c_void;

use crate::wayland::{LibWaylandClient, NULLPTR};

use super::wl_shell_surface::WlShellSurface;
use super::wl_surface::WlSurface;

pub const WL_SHELL_GET_SHELL_SURFACE: u32 = 0;
pub const WL_SHELL_GET_SHELL_SURFACE_SINCE_VERSION: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlShellError {
    /// given wl_surface has another role
    Role = 0,
}

#[repr(C)]
pub struct WlShell([u8; 0]);

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_shell_set_user_data(&self, wl_shell: *mut WlShell, user_data: *mut c_void) {
        (self.wl_proxy_set_user_data)(wl_shell as _, user_data)
    }

    pub unsafe fn wl_shell_get_user_data(&self, wl_shell: *mut WlShell) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_shell as _)
    }

    pub unsafe fn wl_shell_get_version(&self, wl_shell: *mut WlShell) -> u32 {
        (self.wl_proxy_get_version)(wl_shell as _)
    }

    pub unsafe fn wl_shell_destroy(&self, wl_shell: *mut WlShell) {
        (self.wl_proxy_destroy)(wl_shell as _)
    }

    pub unsafe fn wl_shell_get_shell_surface(
        &self,
        wl_shell: *mut WlShell,
        surface: *mut WlSurface,
    ) -> *mut WlShellSurface {
        (self.wl_proxy_marshal_flags)(
            wl_shell as _,
            WL_SHELL_GET_SHELL_SURFACE,
            self.wl_shell_surface_interface,
            (self.wl_proxy_get_version)(wl_shell as _),
            0,
            NULLPTR,
            surface,
        ) as _
    }
}
