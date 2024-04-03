use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, NULLPTR, WL_MARSHAL_FLAG_DESTROY};

use super::wl_buffer::WlBuffer;
use super::wl_callback::WlCallback;
use super::wl_output::WlOutput;
use super::wl_region::WlRegion;

#[repr(C)]
pub struct WlSurface([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlSurfaceError {
    /// buffer scale value is invalid
    InvalidScale = 0,
    /// buffer transform value is invalid
    InvalidTransform = 1,
    /// buffer size is invalid
    InvalidSize = 2,
    /// buffer offset is invalid
    InvalidOffset = 3,
    /// surface was destroyed before its role object
    DefunctRoleObject = 4,
}

pub const WL_SURFACE_DESTROY: u32 = 0;
pub const WL_SURFACE_ATTACH: u32 = 1;
pub const WL_SURFACE_DAMAGE: u32 = 2;
pub const WL_SURFACE_FRAME: u32 = 3;
pub const WL_SURFACE_SET_OPAQUE_REGION: u32 = 4;
pub const WL_SURFACE_SET_INPUT_REGION: u32 = 5;
pub const WL_SURFACE_COMMIT: u32 = 6;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM: u32 = 7;
pub const WL_SURFACE_SET_BUFFER_SCALE: u32 = 8;
pub const WL_SURFACE_DAMAGE_BUFFER: u32 = 9;
pub const WL_SURFACE_OFFSET: u32 = 10;
pub const WL_SURFACE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_PREFERRED_BUFFER_SCALE_SINCE_VERSION: u32 = 6;
pub const WL_SURFACE_PREFERRED_BUFFER_TRANSFORM_SINCE_VERSION: u32 = 6;
pub const WL_SURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ATTACH_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DAMAGE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_OPAQUE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_INPUT_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_COMMIT_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM_SINCE_VERSION: u32 = 2;
pub const WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION: u32 = 3;
pub const WL_SURFACE_DAMAGE_BUFFER_SINCE_VERSION: u32 = 4;
pub const WL_SURFACE_OFFSET_SINCE_VERSION: u32 = 5;

pub struct WlSurfaceListener {
    pub enter: Option<
        unsafe extern "C" fn(
            data: *const c_void,
            wl_surface: *const WlSurface,
            output: *const WlOutput,
        ),
    >,
    pub leave: Option<
        unsafe extern "C" fn(
            data: *const c_void,
            wl_surface: *const WlSurface,
            output: *const WlOutput,
        ),
    >,
    pub preferred_buffer_scale: Option<
        unsafe extern "C" fn(data: *const c_void, wl_surface: *const WlSurface, factor: i32),
    >,
    pub preferred_buffer_transform: Option<
        unsafe extern "C" fn(data: *const c_void, wl_surface: *const WlSurface, transform: u32),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_surface_add_listener(
        &self,
        wl_surface: *mut WlSurface,
        listener: *const WlSurfaceListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_surface as _, listener as _, data)
    }

    pub unsafe fn wl_surface_set_user_data(
        &self,
        wl_surface: *mut WlSurface,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_surface as _, user_data)
    }

    pub unsafe fn wl_surface_get_user_data(&self, wl_surface: *mut WlSurface) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_surface as _)
    }

    pub unsafe fn wl_surface_get_version(&self, wl_surface: *mut WlSurface) -> u32 {
        (self.wl_proxy_get_version)(wl_surface as _)
    }

    pub unsafe fn wl_surface_destroy(&self, wl_surface: *mut WlSurface) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }

    pub unsafe fn wl_surface_attach(
        &self,
        wl_surface: *mut WlSurface,
        buffer: *mut WlBuffer,
        x: i32,
        y: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_ATTACH,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            buffer,
            x,
            y,
        );
    }

    pub unsafe fn wl_surface_damage(
        &self,
        wl_surface: *mut WlSurface,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_DAMAGE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            x,
            y,
            width,
            height,
        );
    }

    pub unsafe fn wl_surface_frame(&self, wl_surface: *mut WlSurface) -> *mut WlCallback {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_FRAME,
            self.wl_callback_interface,
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_surface_set_opaque_region(
        &self,
        wl_surface: *mut WlSurface,
        region: *mut WlRegion,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_SET_OPAQUE_REGION,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            region,
        );
    }

    pub unsafe fn wl_surface_set_input_region(
        &self,
        wl_surface: *mut WlSurface,
        region: *mut WlRegion,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_SET_INPUT_REGION,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            region,
        );
    }

    pub unsafe fn wl_surface_commit(&self, wl_surface: *mut WlSurface) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_COMMIT,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
        );
    }

    pub unsafe fn wl_surface_set_buffer_transform(
        &self,
        wl_surface: *mut WlSurface,
        transform: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_SET_BUFFER_TRANSFORM,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            transform,
        );
    }

    pub unsafe fn wl_surface_set_buffer_scale(&self, wl_surface: *mut WlSurface, scale: i32) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_SET_BUFFER_SCALE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            scale,
        );
    }

    pub unsafe fn wl_surface_damage_buffer(
        &self,
        wl_surface: *mut WlSurface,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_DAMAGE_BUFFER,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            x,
            y,
            width,
            height,
        );
    }

    pub unsafe fn wl_surface_offset(&self, wl_surface: *mut WlSurface, x: i32, y: i32) {
        (self.wl_proxy_marshal_flags)(
            wl_surface as _,
            WL_SURFACE_OFFSET,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_surface as _),
            0,
            x,
            y,
        );
    }
}
