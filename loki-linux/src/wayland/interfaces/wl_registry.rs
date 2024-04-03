use std::ffi::{c_char, c_int, c_void};

use crate::wayland::{LibWaylandClient, WlInterface, NULLPTR};

use super::wl_callback::WlCallback;

pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_REGISTRY_GLOBAL_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL_REMOVE_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlRegistry([u8; 0]);

#[repr(C)]
pub struct WlRegistryListener {
    pub error: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            wl_registry: *mut WlRegistry,
            name: u32,
            interface: *const c_char,
            version: u32,
        ),
    >,
    pub delete_id:
        Option<unsafe extern "C" fn(data: *mut c_void, wl_registry: *mut WlRegistry, name: u32)>,
}

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_registry_add_listener(
        &self,
        wl_registry: *mut WlRegistry,
        listener: *const WlRegistryListener,
        data: *mut c_void,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_registry as _, listener as _, data)
    }

    pub unsafe fn wl_registry_set_user_data(
        &self,
        wl_registry: *mut WlRegistry,
        user_data: *mut c_void,
    ) {
        (self.wl_proxy_set_user_data)(wl_registry as _, user_data)
    }

    pub unsafe fn wl_registry_get_user_data(&self, wl_registry: *mut WlRegistry) -> *mut c_void {
        (self.wl_proxy_get_user_data)(wl_registry as _)
    }

    pub unsafe fn wl_registry_get_version(&self, wl_registry: *mut WlRegistry) -> u32 {
        (self.wl_proxy_get_version)(wl_registry as _)
    }

    pub unsafe fn wl_registry_destroy(&self, wl_registry: *mut WlRegistry) {
        (self.wl_proxy_destroy)(wl_registry as _)
    }

    pub unsafe fn wl_registry_bind(
        &self,
        wl_registry: *mut WlRegistry,
        name: u32,
        interface: *const WlInterface,
        version: u32,
    ) -> *mut c_void {
        (self.wl_proxy_marshal_flags)(
            wl_registry as _,
            WL_REGISTRY_BIND,
            interface,
            version,
            0,
            name,
            (*interface).name,
            version,
            NULLPTR,
        ) as _
    }
}
