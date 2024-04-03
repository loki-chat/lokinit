use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, WL_MARSHAL_FLAG_DESTROY};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WlDataOfferError {
    /// finish request was called untimely
    InvalidFinish = 0,
    /// action mask contains invalid values
    InvalidActionMask = 1,
    /// action argument has an invalid value
    InvalidAction = 2,
    /// offer doesn't accept this request
    InvalidOffer = 3,
}

const WL_DATA_OFFER_ACCEPT: u32 = 0;
const WL_DATA_OFFER_RECEIVE: u32 = 1;
const WL_DATA_OFFER_DESTROY: u32 = 2;
const WL_DATA_OFFER_FINISH: u32 = 3;
const WL_DATA_OFFER_SET_ACTIONS: u32 = 4;
const WL_DATA_OFFER_OFFER_SINCE_VERSION: u32 = 1;
const WL_DATA_OFFER_SOURCE_ACTIONS_SINCE_VERSION: u32 = 3;
const WL_DATA_OFFER_ACTION_SINCE_VERSION: u32 = 3;
const WL_DATA_OFFER_ACCEPT_SINCE_VERSION: u32 = 1;
const WL_DATA_OFFER_RECEIVE_SINCE_VERSION: u32 = 1;
const WL_DATA_OFFER_DESTROY_SINCE_VERSION: u32 = 1;
const WL_DATA_OFFER_FINISH_SINCE_VERSION: u32 = 3;
const WL_DATA_OFFER_SET_ACTIONS_SINCE_VERSION: u32 = 3;

#[repr(C)]
pub struct WlDataOffer([u8; 0]);

pub struct WlDataOfferListener {
    pub offer: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_offer: *mut WlDataOffer,
            mime_type: *const c_char,
        ),
    >,
    pub source_actions: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_data_offer: *mut WlDataOffer,
            source_actions: u32,
        ),
    >,
    pub action: Option<
        unsafe extern "C" fn(data: *mut c_void, wl_data_offer: *mut WlDataOffer, dnd_action: u32),
    >,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_data_offer_add_listener(
        &self,
        wl_data_offer: *mut WlDataOffer,
        listener: *const WlDataOfferListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_data_offer as _, listener as _, data)
    }

    pub unsafe fn wl_data_offer_set_user_data(
        &self,
        wl_data_offer: *mut WlDataOffer,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_data_offer as _, user_data)
    }

    pub unsafe fn wl_data_offer_get_user_data(
        &self,
        wl_data_offer: *mut WlDataOffer,
    ) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_data_offer as _)
    }

    pub unsafe fn wl_data_offer_get_version(&self, wl_data_offer: *mut WlDataOffer) -> u32 {
        (self.wl_proxy_get_version)(wl_data_offer as _)
    }

    pub unsafe fn wl_data_offer_accept(
        &self,
        wl_data_offer: *mut WlDataOffer,
        serial: u32,
        mime_type: *const c_char,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_offer as _,
            WL_DATA_OFFER_ACCEPT,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_offer as _),
            0,
            serial,
            mime_type,
        );
    }

    pub unsafe fn wl_data_offer_receive(
        &self,
        wl_data_offer: *mut WlDataOffer,
        mime_type: *const c_char,
        fd: i32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_offer as _,
            WL_DATA_OFFER_RECEIVE,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_offer as _),
            0,
            mime_type,
            fd,
        );
    }

    pub unsafe fn wl_data_offer_destroy(&self, wl_data_offer: *mut WlDataOffer) {
        (self.wl_proxy_marshal_flags)(
            wl_data_offer as _,
            WL_DATA_OFFER_DESTROY,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_offer as _),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }

    pub unsafe fn wl_data_offer_finish(&self, wl_data_offer: *mut WlDataOffer) {
        (self.wl_proxy_marshal_flags)(
            wl_data_offer as _,
            WL_DATA_OFFER_FINISH,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_offer as _),
            0,
        );
    }

    pub unsafe fn wl_data_offer_set_actions(
        &self,
        wl_data_offer: *mut WlDataOffer,
        dnd_actions: u32,
        preferred_action: u32,
    ) {
        (self.wl_proxy_marshal_flags)(
            wl_data_offer as _,
            WL_DATA_OFFER_SET_ACTIONS,
            std::ptr::null(),
            (self.wl_proxy_get_version)(wl_data_offer as _),
            0,
            dnd_actions,
            preferred_action,
        );
    }
}
