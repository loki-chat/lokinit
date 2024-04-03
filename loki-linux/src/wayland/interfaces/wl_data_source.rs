use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, WL_MARSHAL_FLAG_DESTROY};

pub const WL_DATA_SOURCE_OFFER: u32 = 0;
pub const WL_DATA_SOURCE_DESTROY: u32 = 1;
pub const WL_DATA_SOURCE_SET_ACTIONS: u32 = 2;
pub const WL_DATA_SOURCE_TARGET_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SEND_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_CANCELLED_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DND_DROP_PERFORMED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_DND_FINISHED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SET_ACTIONS_SINCE_VERSION: u32 = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlDataSourceError {
    /// action mask contains invalid values
    InvalidActionMask = 0,
    /// source doesn't accept this request
    InvalidSource = 1,
}

#[repr(C)]
pub struct WlDataSource([u8; 0]);

pub struct WlDataSourceListener {
    pub target: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_source: *mut WlDataSource,
            mime_type: *const c_char,
        ),
    >,
    pub send: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_source: *mut WlDataSource,
            mime_type: *const c_char,
            fd: i32,
        ),
    >,
    pub cancelled:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_data_source: *mut WlDataSource)>,
    pub dnd_drop_performed:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_data_source: *mut WlDataSource)>,
    pub dnd_finished:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_data_source: *mut WlDataSource)>,
    pub action: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_data_source: *mut WlDataSource, dnd_action: u32),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_data_source_add_listener(
        &self,
        wl_data_source: *mut WlDataSource,
        listener: *const WlDataSourceListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_data_source as _, listener as _, data)
    }

    pub unsafe fn wl_data_source_set_user_data(
        &self,
        wl_data_source: *mut WlDataSource,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_data_source as _, user_data)
    }

    pub unsafe fn wl_data_source_get_user_data(
        &self,
        wl_data_source: *mut WlDataSource,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_data_source as _)
    }

    pub unsafe fn wl_data_source_get_version(&self, wl_data_source: *mut WlDataSource) -> u32 {
        (self.wl_proxy_get_version)(wl_data_source as _)
    }

    pub unsafe fn wl_data_source_offer(
        &self,
        wl_data_source: *mut WlDataSource,
        mime_type: *const c_char,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_source as _,
            WL_DATA_SOURCE_OFFER,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_source as _),
            0,
            mime_type,
        );
    }

    pub unsafe fn wl_data_source_destroy(&self, wl_data_source: *mut WlDataSource) {
        (self.wl_proxy_marshal_flags)(
            wl_data_source as _,
            WL_DATA_SOURCE_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_source as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }

    pub unsafe fn wl_data_source_set_actions(
        &self,
        wl_data_source: *mut WlDataSource,
        dnd_actions: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_source as _,
            WL_DATA_SOURCE_SET_ACTIONS,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_source as _),
            0,
            dnd_actions,
        );
    }
}
