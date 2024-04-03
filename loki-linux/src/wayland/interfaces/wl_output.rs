use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, WL_MARSHAL_FLAG_DESTROY};

#[repr(C)]
pub struct WlOutput([u8; 0]);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlOutputSubpixel {
    /// unknown geometry
    Unknown = 0,
    /// no geometry
    None = 1,
    /// horizontal RGB
    HorizontalRgb = 2,
    /// horizontal BGR
    HorizontalBgr = 3,
    /// vertical RGB
    VerticalRgb = 4,
    /// vertical BGR
    VerticalBgr = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlOutputTransform {
    /// no transform
    Normal = 0,
    /// 90 degrees counter-clockwise
    Deg90 = 1,
    /// 180 degrees counter-clockwise
    Deg180 = 2,
    /// 270 degrees counter-clockwise
    Deg270 = 3,
    /// 180 degree flip around a vertical axis
    Flipped = 4,
    /// flip and rotate 90 degrees counter-clockwise
    Flipped90 = 5,
    /// flip and rotate 180 degrees counter-clockwise
    Flipped180 = 6,
    /// flip and rotate 270 degrees counter-clockwise
    Flipped270 = 7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlOutputMode {
    /// indicates this is the current mode
    Current = 0x1,
    /// indicates this is the preferred mode
    Preferred = 0x2,
}

pub const WL_OUTPUT_RELEASE: u32 = 0;
pub const WL_OUTPUT_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_MODE_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_DONE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_SCALE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_NAME_SINCE_VERSION: u32 = 4;
pub const WL_OUTPUT_DESCRIPTION_SINCE_VERSION: u32 = 4;
pub const WL_OUTPUT_RELEASE_SINCE_VERSION: u32 = 3;

pub struct WlOutputListener {
    pub geometry: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_output: *mut WlOutput,
            x: i32,
            y: i32,
            physical_width: i32,
            physical_height: i32,
            subpixel: i32,
            make: *const c_char,
            model: *const c_char,
            transform: i32,
        ),
    >,

    pub mode: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_output: *mut WlOutput,
            flags: u32,
            width: i32,
            height: i32,
            refresh: i32,
        ),
    >,

    pub done: Option<unsafe extern "C" fn(data: *mut c_void, wl_output: *mut WlOutput)>,

    pub scale:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_output: *mut WlOutput, factor: i32)>,

    pub name: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_output: *mut WlOutput, name: *const c_char),
    >,

    pub description: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_output: *mut WlOutput,
            description: *const c_char,
        ),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_output_add_listener(
        &self,
        wl_output: *mut WlOutput,
        listener: *const WlOutputListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_output as _, listener as _, data)
    }

    pub unsafe fn wl_output_set_user_data(
        &self,
        wl_output: *mut WlOutput,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_output as _, user_data)
    }

    pub unsafe fn wl_output_get_user_data(&self, wl_output: *mut WlOutput) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_output as _)
    }

    pub unsafe fn wl_output_get_version(&self, wl_output: *mut WlOutput) -> u32 {
        (self.wl_proxy_get_version)(wl_output as _)
    }

    pub unsafe fn wl_output_destroy(&self, wl_output: *mut WlOutput) {
        (self.wl_proxy_destroy)(wl_output as _)
    }

    pub unsafe fn wl_output_release(&self, wl_output: *mut WlOutput) {
        (self.wl_proxy_marshal_flags)(
            wl_output as _,
            WL_OUTPUT_RELEASE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_output as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}