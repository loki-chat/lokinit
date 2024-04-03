#![allow(unused)]

pub mod interfaces;

use std::ffi::{c_char, c_int, c_void};

use crate::library;

use self::interfaces::wl_display::WlDisplay;
use self::interfaces::wl_registry::{WlRegistry, WL_REGISTRY_BIND};

pub(crate) const NULLPTR: *mut c_void = std::ptr::null_mut::<c_void>();

pub const WL_MARSHAL_FLAG_DESTROY: u32 = 1 << 0;

pub type WlFixed = i32;

#[repr(C)]
pub struct WlProxy([u8; 0]);

#[repr(C)]
pub struct WlEventQueue([u8; 0]);

#[repr(C)]
pub struct WlObject([u8; 0]);

#[repr(C)]
pub struct WlMessage([u8; 0]);

#[repr(C)]
pub struct WlArray {
    /// Array size
    pub size: usize,
    /// Allocated space
    pub alloc: usize,
    /// Array data
    pub data: *mut c_void,
}

#[repr(C)]
pub union WlArgument {
    /// `int`
    pub i: i32,
    /// `uint`
    pub u: u32,
    /// `fixed`
    pub f: i32,
    /// `string`
    pub s: *const c_char,
    /// `object`
    pub o: *mut WlObject,
    /// `new_id`
    pub n: u32,
    /// `array`
    pub a: *mut WlArray,
    /// `fd`
    pub h: i32,
}

#[repr(C)]
pub struct WlInterface {
    pub name: *const c_char,
    pub ver: c_int,
    pub method_count: c_int,
    pub methods: *const WlMessage,
    pub event_count: c_int,
    pub events: *const WlMessage,
}

pub type GlobalRegistryHandler =
    unsafe extern "C" fn(*mut c_void, *mut WlRegistry, u32, *const c_char, u32);
pub type GlobalRegistryRemover = unsafe extern "C" fn(*mut c_void, *mut WlRegistry, u32);

pub type WlDispatcherFunc = unsafe extern "C" fn(
    user_data: *const c_void,
    target: *mut c_void,
    opcode: u32,
    msg: *const WlMessage,
    args: *mut WlArgument,
) -> c_int;

#[repr(C)]
pub struct WlRegistryListener {
    pub global_registry_handler: GlobalRegistryHandler,
    pub global_registry_remover: GlobalRegistryRemover,
}

library! {
    [LibWaylandClient <-> "wayland-client"] ;

    {
        // interfaces

        pub wl_buffer_interface: *mut WlInterface;
        pub wl_callback_interface: *mut WlInterface;
        pub wl_compositor_interface: *mut WlInterface;
        pub wl_data_device_interface: *mut WlInterface;
        pub wl_data_device_manager_interface: *mut WlInterface;
        pub wl_data_offer_interface: *mut WlInterface;
        pub wl_data_source_interface: *mut WlInterface;
        pub wl_display_interface: *mut WlInterface;
        pub wl_keyboard_interface: *mut WlInterface;
        pub wl_output_interface: *mut WlInterface;
        pub wl_pointer_interface: *mut WlInterface;
        pub wl_region_interface: *mut WlInterface;
        pub wl_registry_interface: *mut WlInterface;
        pub wl_seat_interface: *mut WlInterface;
        pub wl_shell_interface: *mut WlInterface;
        pub wl_shell_surface_interface: *mut WlInterface;
        pub wl_shm_interface: *mut WlInterface;
        pub wl_shm_pool_interface: *mut WlInterface;
        pub wl_subcompositor_interface: *mut WlInterface;
        pub wl_subsurface_interface: *mut WlInterface;
        pub wl_surface_interface: *mut WlInterface;
        pub wl_touch_interface: *mut WlInterface;
    }

    // wayland-client-core

    pub fn wl_event_queue_destroy(queue: *mut WlEventQueue);

    pub fn wl_proxy_marshal_flags(proxy: *mut WlProxy, opcode: u32, interface: *const WlInterface, version: u32, flags: u32, ...) -> *mut WlProxy;
    pub fn wl_proxy_marshal_array_flags(proxy: *mut WlProxy, opcode: u32, interface: *const WlInterface, version: u32, flags: u32, args: *mut WlArgument) -> *mut WlProxy;
    pub fn wl_proxy_marshal(p: *mut WlProxy, opcode: u32, ...);
    pub fn wl_proxy_marshal_array(p: *mut WlProxy, opcode: u32, args: *mut WlArgument);
    pub fn wl_proxy_create(factory: *mut WlProxy, interface: *const WlInterface) -> *mut WlProxy;
    pub fn wl_proxy_create_wrapper(proxy: *mut c_void) -> *mut c_void;
    pub fn wl_proxy_wrapper_destroy(proxy_wrapper: *mut c_void);
    pub fn wl_proxy_marshal_constructor(proxy: *mut WlProxy, opcode: u32, interface: *const WlInterface, ...) -> *mut WlProxy;
    pub fn wl_proxy_marshal_constructor_versioned(proxy: *mut WlProxy, opcode: u32, interface: *const WlInterface, version: u32, ...) -> *mut WlProxy;
    pub fn wl_proxy_marshal_array_constructor(proxy: *mut WlProxy, opcode: u32, args: *mut WlArgument, interface: *const WlInterface) -> *mut WlProxy;
    pub fn wl_proxy_marshal_array_constructor_versioned(proxy: *mut WlProxy, opcode: u32, args: *mut WlArgument, interface: *const WlInterface, version: u32) -> *mut WlProxy;
    pub fn wl_proxy_destroy(proxy: *mut WlProxy);
    pub fn wl_proxy_add_listener(proxy: *mut WlProxy, implementation: *mut Option<unsafe extern "C" fn()>, data: *mut c_void) -> c_int;
    pub fn wl_proxy_get_listener(proxy: *mut WlProxy) -> *mut c_void;
    pub fn wl_proxy_add_dispatcher(proxy: *mut WlProxy, dispatcher_func: Option<WlDispatcherFunc>, dispatcher_data: *const c_void, data: *mut c_void) -> c_int;
    pub fn wl_proxy_set_user_data(proxy: *mut WlProxy, user_data: *mut c_void);
    pub fn wl_proxy_get_user_data(proxy: *mut WlProxy) -> *mut c_void;
    pub fn wl_proxy_get_version(proxy: *mut WlProxy) -> u32;
    pub fn wl_proxy_get_id(proxy: *mut WlProxy) -> u32;
    pub fn wl_proxy_set_tag(proxy: *mut WlProxy, tag: *const *const c_char);
    pub fn wl_proxy_get_tag(proxy: *mut WlProxy) -> *const *const c_char;
    pub fn wl_proxy_get_class(proxy: *mut WlProxy) -> *const c_char;
    pub fn wl_proxy_set_queue(proxy: *mut WlProxy, queue: *mut WlEventQueue);

    pub fn wl_display_connect(name: *const c_char) -> *mut WlDisplay;
    pub fn wl_display_connect_to_fd(fd: c_int) -> *mut WlDisplay;
    pub fn wl_display_disconnect(display: *mut WlDisplay);
    pub fn wl_display_get_fd(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_dispatch(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_dispatch_queue(display: *mut WlDisplay, queue: *mut WlEventQueue) -> c_int;
    pub fn wl_display_dispatch_queue_pending(display: *mut WlDisplay, queue: *mut WlEventQueue) -> c_int;
    pub fn wl_display_dispatch_pending(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_get_error(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_get_protocol_error(display: *mut WlDisplay, interface: *mut *const WlInterface, id: *mut u32) -> u32;
    pub fn wl_display_flush(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_roundtrip_queue(display: *mut WlDisplay, queue: *mut WlEventQueue) -> c_int;
    pub fn wl_display_roundtrip(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_create_queue(display: *mut WlDisplay) -> *mut WlEventQueue;
    pub fn wl_display_prepare_read_queue(display: *mut WlDisplay, queue: *mut WlEventQueue) -> c_int;
    pub fn wl_display_prepare_read(display: *mut WlDisplay) -> c_int;
    pub fn wl_display_cancel_read(display: *mut WlDisplay);
    pub fn wl_display_read_events(display: *mut WlDisplay) -> c_int;

    // pub fn wl_log_set_handler_client(handler: WlLogFunc);
}
