use std::ffi::c_void;

use crate::wayland::{LibWaylandClient, NULLPTR};

use super::wl_data_device::WlDataDevice;
use super::wl_data_source::WlDataSource;
use super::wl_seat::WlSeat;

pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: u32 = 0;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE_SINCE_VERSION: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlDataDeviceManagerDndAction {
    /// no action
    None = 0,
    /// copy action
    Copy = 1,
    /// move action
    Move = 2,
    /// ask action
    Ask = 4,
}

#[repr(C)]
pub struct WlDataDeviceManager([u8; 0]);

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_data_device_manager_set_user_data(
        &self,
        wl_data_device_manager: *mut WlDataDeviceManager,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_data_device_manager as _, user_data)
    }

    pub unsafe fn wl_data_device_manager_get_user_data(
        &self,
        wl_data_device_manager: *mut WlDataDeviceManager,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_data_device_manager as _)
    }

    pub unsafe fn wl_data_device_manager_get_version(
        &self,
        wl_data_device_manager: *mut WlDataDeviceManager,
    ) -> u32 {
        (self.wl_proxy_get_version)(wl_data_device_manager as _)
    }

    pub unsafe fn wl_data_device_manager_destroy(
        &self,
        wl_data_device_manager: *mut WlDataDeviceManager,
    ) {
        (self.wl_proxy_destroy)(wl_data_device_manager as _)
    }

    pub unsafe fn wl_data_device_manager_create_data_source(
        &self,
        wl_data_device_manager: *mut WlDataDeviceManager,
    ) -> *mut WlDataSource {
        (self.wl_proxy_marshal_flags)(
            wl_data_device_manager as _,
            WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE,
            self.wl_data_source_interface,
            (self.wl_proxy_get_version)(wl_data_device_manager as _),
            0,
            NULLPTR,
        ) as _
    }

    pub unsafe fn wl_data_device_manager_get_data_device(
        &self,
        wl_data_device_manager: *mut WlDataDeviceManager,
        seat: *mut WlSeat,
    ) -> *mut WlDataDevice {
        (self.wl_proxy_marshal_flags)(
            wl_data_device_manager as _,
            WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE,
            self.wl_data_device_interface,
            (self.wl_proxy_get_version)(wl_data_device_manager as _),
            0,
            NULLPTR,
            seat,
        ) as _
    }
}
