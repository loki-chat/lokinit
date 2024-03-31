#![allow(unused)]

pub mod requests;

use std::ffi::{c_char, c_int, c_void};

use crate::library;

pub const WL_DISPLAY_SYNC: u32 = 0;
pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_DISPLAY_ERROR_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_DELETE_ID_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_GET_REGISTRY_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_REGISTRY_GLOBAL_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL_REMOVE_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND_SINCE_VERSION: u32 = 1;
pub const WL_CALLBACK_DONE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_COMPOSITOR_CREATE_REGION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_CREATE_BUFFER: u32 = 0;
pub const WL_SHM_POOL_DESTROY: u32 = 1;
pub const WL_SHM_POOL_RESIZE: u32 = 2;
pub const WL_SHM_POOL_CREATE_BUFFER_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SHM_CREATE_POOL: u32 = 0;
pub const WL_SHM_FORMAT_SINCE_VERSION: u32 = 1;
pub const WL_SHM_CREATE_POOL_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_DESTROY: u32 = 0;
pub const WL_BUFFER_RELEASE_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ACCEPT: u32 = 0;
pub const WL_DATA_OFFER_RECEIVE: u32 = 1;
pub const WL_DATA_OFFER_DESTROY: u32 = 2;
pub const WL_DATA_OFFER_FINISH: u32 = 3;
pub const WL_DATA_OFFER_SET_ACTIONS: u32 = 4;
pub const WL_DATA_OFFER_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_SOURCE_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_ACCEPT_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_RECEIVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_FINISH_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_SET_ACTIONS_SINCE_VERSION: u32 = 3;
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
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: u32 = 0;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_GET_SHELL_SURFACE: u32 = 0;
pub const WL_SHELL_GET_SHELL_SURFACE_SINCE_VERSION: u32 = 1;
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
pub const WL_SURFACE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_LEAVE_SINCE_VERSION: u32 = 1;
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
pub const WL_SEAT_GET_POINTER: u32 = 0;
pub const WL_SEAT_GET_KEYBOARD: u32 = 1;
pub const WL_SEAT_GET_TOUCH: u32 = 2;
pub const WL_SEAT_RELEASE: u32 = 3;
pub const WL_SEAT_CAPABILITIES_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_NAME_SINCE_VERSION: u32 = 2;
pub const WL_SEAT_GET_POINTER_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_KEYBOARD_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_TOUCH_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_RELEASE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT_SINCE_VERSION: u32 = 6;
pub const WL_POINTER_SET_CURSOR: u32 = 0;
pub const WL_POINTER_RELEASE: u32 = 1;
pub const WL_POINTER_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_BUTTON_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_AXIS_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_FRAME_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_STOP_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_DISCRETE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_SET_CURSOR_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_KEYBOARD_RELEASE: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_MODIFIERS_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_REPEAT_INFO_SINCE_VERSION: u32 = 4;
pub const WL_KEYBOARD_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_TOUCH_RELEASE: u32 = 0;
pub const WL_TOUCH_DOWN_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_UP_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_CANCEL_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_SHAPE_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_ORIENTATION_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_OUTPUT_RELEASE: u32 = 0;
pub const WL_OUTPUT_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_MODE_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_DONE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_SCALE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_REGION_DESTROY: u32 = 0;
pub const WL_REGION_ADD: u32 = 1;
pub const WL_REGION_SUBTRACT: u32 = 2;
pub const WL_REGION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_REGION_ADD_SINCE_VERSION: u32 = 1;
pub const WL_REGION_SUBTRACT_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_DESTROY: u32 = 0;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE: u32 = 1;
pub const WL_SUBCOMPOSITOR_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_DESTROY: u32 = 0;
pub const WL_SUBSURFACE_SET_POSITION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE: u32 = 2;
pub const WL_SUBSURFACE_PLACE_BELOW: u32 = 3;
pub const WL_SUBSURFACE_SET_SYNC: u32 = 4;
pub const WL_SUBSURFACE_SET_DESYNC: u32 = 5;
pub const WL_SUBSURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_POSITION_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_ABOVE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_BELOW_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_DESYNC_SINCE_VERSION: u32 = 1;

#[repr(C)]
pub struct WlProxy([u8; 0]);

#[repr(C)]
pub struct WlEventQueue([u8; 0]);

#[repr(C)]
pub struct WlObject([u8; 0]);

#[repr(C)]
pub struct WlDisplay([u8; 0]);

#[repr(C)]
pub struct WlRegistry([u8; 0]);

#[repr(C)]
pub struct WlCompositor([u8; 0]);

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

#[allow(clippy::missing_safety_doc)]
impl LibWaylandClient {
    pub unsafe fn wl_registry_add_listener<T>(
        &self,
        wl_registry: *mut WlRegistry,
        listener: *const WlRegistryListener,
        data: *mut T,
    ) -> c_int {
        (self.wl_proxy_add_listener)(wl_registry as _, listener as _, data as _)
    }

    pub unsafe fn wl_registry_bind(
        &self,
        wl_registry: *mut WlRegistry,
        id: u32,
        interface: *const WlInterface,
        version: u32,
    ) -> *mut c_void {
        let id: *mut WlProxy = (self.wl_proxy_marshal_flags)(
            wl_registry as _,
            WL_REGISTRY_BIND,
            interface,
            version,
            0,
            id,
            (*interface).name,
            version,
            0,
        );

        id as _
    }
}
