use std::ffi::{c_int, c_void};

use crate::wayland::{LibWaylandClient, WlFixed, WL_MARSHAL_FLAG_DESTROY};

use super::wl_data_offer::WlDataOffer;
use super::wl_data_source::WlDataSource;
use super::wl_surface::WlSurface;

pub const WL_DATA_DEVICE_START_DRAG: u32 = 0;
pub const WL_DATA_DEVICE_SET_SELECTION: u32 = 1;
pub const WL_DATA_DEVICE_RELEASE: u32 = 2;
pub const WL_DATA_DEVICE_DATA_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_DROP_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_START_DRAG_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_SET_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_RELEASE_SINCE_VERSION: u32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlDataDeviceError {
    /// given wl_surface has another role
    Role = 0,
}

#[repr(C)]
pub struct WlDataDevice([u8; 0]);

pub struct WlDataDeviceListener {
    data_offer: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_device: *mut WlDataDevice,
            id: *mut WlDataDevice,
        ),
    >,
    enter: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_device: *mut WlDataDevice,
            serial: u32,
            surface: *mut WlSurface,
            x: WlFixed,
            y: WlFixed,
        ),
    >,
    leave: Option<unsafe extern "C" fn(data: *mut c_void, wl_data_device: *mut WlDataDevice)>,
    motion: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_device: *mut WlDataDevice,
            time: u32,
            x: WlFixed,
            y: WlFixed,
        ),
    >,
    drop: Option<unsafe extern "C" fn(data: *mut c_void, wl_data_device: *mut WlDataDevice)>,
    selection: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_device: *mut WlDataDevice,
            id: *mut WlDataOffer,
        ),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_data_device_add_listener(
        &self,
        wl_data_device: *mut WlDataDevice,
        listener: *const WlDataDeviceListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_data_device as _, listener as _, data)
    }

    pub unsafe fn wl_data_device_set_user_data(
        &self,
        wl_data_device: *mut WlDataDevice,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_data_device as _, user_data)
    }

    pub unsafe fn wl_data_device_get_user_data(
        &self,
        wl_data_device: *mut WlDataDevice,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_data_device as _)
    }

    pub unsafe fn wl_data_device_get_version(&self, wl_data_device: *mut WlDataDevice) -> u32 {
        (self.wl_proxy_get_version)(wl_data_device as _)
    }

    pub unsafe fn wl_data_device_destroy(&self, wl_data_device: *mut WlDataDevice) {
        (self.wl_proxy_destroy)(wl_data_device as _)
    }

    pub unsafe fn wl_data_device_start_drag(
        &self,
        wl_data_device: *mut WlDataDevice,
        source: *mut WlDataSource,
        origin: *mut WlSurface,
        icon: *mut WlSurface,
        serial: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_device as _,
            WL_DATA_DEVICE_START_DRAG,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_device as _),
            0,
            source,
            origin,
            icon,
            serial,
        );
    }

    pub unsafe fn wl_data_device_set_selection(
        &self,
        wl_data_device: *mut WlDataDevice,
        source: *mut WlDataSource,
        serial: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_device as _,
            WL_DATA_DEVICE_SET_SELECTION,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_device as _),
            0,
            source,
            serial,
        );
    }

    pub unsafe fn wl_data_device_release(&self, wl_data_device: *mut WlDataDevice) {
        (self.wl_proxy_marshal_flags)(
            wl_data_device as _,
            WL_DATA_DEVICE_RELEASE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_device as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}
