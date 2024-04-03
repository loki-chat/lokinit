use std::ffi::{c_char, c_int, c_void};

use crate::wayland::LibWaylandClient;

use super::wl_output::WlOutput;
use super::wl_seat::WlSeat;
use super::wl_surface::WlSurface;

#[repr(C)]
pub struct WlShellSurface([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlShellSurfaceResize {
    /// no edge
    None = 0,
    /// top edge
    Top = 1,
    /// bottom edge
    Bottom = 2,
    /// left edge
    Left = 4,
    /// top and left edges
    TopLeft = 5,
    /// bottom and left edges
    BottomLeft = 6,
    /// right edge
    Right = 8,
    /// top and right edges
    TopRight = 9,
    /// bottom and right edges
    BottomRight = 10,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlShellSurfaceTransient {
    /// do not set keyboard focus
    TransientInactive = 0x1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlShellSurfaceFullscreenMethod {
    /// no preference, apply default policy
    MethodDefault = 0,
    /// scale, preserve the surface's aspect ratio and center on output
    MethodScale = 1,
    /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
    MethodDriver = 2,
    /// no upscaling, center on output and add black borders to compensate size mismatch
    MethodFill = 3,
}

pub const WL_SHELL_SURFACE_PONG: u32 = 0;
pub const WL_SHELL_SURFACE_MOVE: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE: u32 = 2;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL: u32 = 3;
pub const WL_SHELL_SURFACE_SET_TRANSIENT: u32 = 4;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN: u32 = 5;
pub const WL_SHELL_SURFACE_SET_POPUP: u32 = 6;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED: u32 = 7;
pub const WL_SHELL_SURFACE_SET_TITLE: u32 = 8;
pub const WL_SHELL_SURFACE_SET_CLASS: u32 = 9;
pub const WL_SHELL_SURFACE_PING_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_POPUP_DONE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PONG_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_MOVE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TRANSIENT_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_POPUP_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TITLE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_CLASS_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlShellSurfaceListener {
    pub ping: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_shell_surface: *mut WlShellSurface, serial: u32),
    >,
    pub configure: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_shell_surface: *mut WlShellSurface,
            edges: u32,
            width: i32,
            height: i32,
        ),
    >,
    pub popup_done:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_shell_surface: *mut WlShellSurface)>,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_shell_surface_add_listener(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        listener: *const WlShellSurfaceListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_shell_surface as _, listener as _, data)
    }

    pub unsafe fn wl_shell_surface_set_user_data(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_shell_surface as _, user_data)
    }

    pub unsafe fn wl_shell_surface_get_user_data(
        &self,
        wl_shell_surface: *mut WlShellSurface,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_shell_surface as _)
    }

    pub unsafe fn wl_shell_surface_get_version(
        &self,
        wl_shell_surface: *mut WlShellSurface,
    ) -> u32 {
        (self.wl_proxy_get_version)(wl_shell_surface as _)
    }

    pub unsafe fn wl_shell_surface_destroy(&self, wl_shell_surface: *mut WlShellSurface) {
        (self.wl_proxy_destroy)(wl_shell_surface as _)
    }

    pub unsafe fn wl_shell_surface_pong(&self, wl_shell_surface: *mut WlShellSurface, serial: u32) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_PONG,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            serial,
        );
    }

    pub unsafe fn wl_shell_surface_move(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        seat: *mut WlSeat,
        serial: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_MOVE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            seat,
            serial,
        );
    }

    pub unsafe fn wl_shell_surface_resize(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        seat: *mut WlSeat,
        serial: u32,
        edges: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_RESIZE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            seat,
            serial,
            edges,
        );
    }

    pub unsafe fn wl_shell_surface_set_toplevel(&self, wl_shell_surface: *mut WlShellSurface) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_TOPLEVEL,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
        );
    }

    pub unsafe fn wl_shell_surface_set_transient(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        parent: *mut WlSurface,
        x: i32,
        y: i32,
        flags: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_TRANSIENT,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            parent,
            x,
            y,
            flags,
        );
    }

    pub unsafe fn wl_shell_surface_set_fullscreen(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        method: u32,
        framerate: u32,
        output: *mut WlOutput,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_FULLSCREEN,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            method,
            framerate,
            output,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub unsafe fn wl_shell_surface_set_popup(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        seat: *mut WlSeat,
        serial: u32,
        parent: *mut WlSurface,
        x: i32,
        y: i32,
        flags: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_POPUP,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            seat,
            serial,
            parent,
            x,
            y,
            flags,
        );
    }

    pub unsafe fn wl_shell_surface_set_maximized(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        output: *mut WlOutput,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_MAXIMIZED,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            output,
        );
    }

    pub unsafe fn wl_shell_surface_set_title(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        title: *const c_char,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_TITLE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            title,
        );
    }

    pub unsafe fn wl_shell_surface_set_class(
        &self,
        wl_shell_surface: *mut WlShellSurface,
        class_: *const c_char,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_shell_surface as _,
            WL_SHELL_SURFACE_SET_CLASS,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_shell_surface as _),
            0,
            class_,
        );
    }
}
