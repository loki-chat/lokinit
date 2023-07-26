
use std::{ffi::{CString, c_void, c_char, CStr}, ptr::null_mut, marker::PhantomData};

use crate::library;

use super::LoadingError;

struct StaticAddr<T>(usize, PhantomData<T>);
impl <T> StaticAddr<T> {
    pub const fn null() -> Self {
        StaticAddr(0, PhantomData)
    }

    pub fn set(&mut self, display: *mut T) {
        self.0 = display as usize;
    }

    pub fn as_mut(&self) -> *mut T {
        self.0 as *mut T
    }
}
unsafe impl <T> Send for StaticAddr<T> {}
unsafe impl <T> Sync for StaticAddr<T> {}

static mut DISPLAY: StaticAddr<WlDisplay> = StaticAddr::null();
static mut COMPOSITOR: StaticAddr<WlCompositor> = StaticAddr::null();
static mut LIB_WAYLAND_CLIENT: Option<LibWaylandClient> = None;

macro_rules! lib_wlc {
    ($field:ident) => {
        (LIB_WAYLAND_CLIENT.as_ref().unwrap().$field)
    };
}

#[derive(Debug)]
pub enum WaylandInitError {
    NoLibWayland(LoadingError),

    NoDisplaySet,
    InvalidDisplayFormat,
    CannotOpenDisplay(String),
}
impl std::error::Error for WaylandInitError {}
impl std::fmt::Display for WaylandInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::NoLibWayland(err) => format!("Cannot find libwayland-client: {err:?}"),
            Self::NoDisplaySet => "WAYLAND_DISPLAY environment variable is not set".to_owned(),
            Self::InvalidDisplayFormat => "WAYLAND_DISPLAY is not a valid C string".to_owned(),
            Self::CannotOpenDisplay(display) => format!("Cannot find Wayland display: '{display}'"),
        })
    }
}

impl From<LoadingError> for WaylandInitError {
    fn from(value: LoadingError) -> Self {
        Self::NoLibWayland(value)
    }
}

pub struct WaylandBackend {
    display: *mut WlDisplay,

    #[allow(unused)]
    listener: Box<WlRegistryListener>,
}
impl WaylandBackend {
    pub fn init() -> Result<Self, WaylandInitError> {
        let display = std::env::var("WAYLAND_DISPLAY").map_err(|_| WaylandInitError::NoDisplaySet)?;
        let display_c = CString::new(display.clone()).map_err(|_| WaylandInitError::InvalidDisplayFormat)?;
        let display_c = display_c.to_bytes_with_nul() as *const _ as *const c_char;

        unsafe {
            LIB_WAYLAND_CLIENT = Some(LibWaylandClient::new()?);
        };

        let display_ref = unsafe { lib_wlc!(wl_display_connect)(display_c) };
        if display_ref.is_null() {
            return Err(WaylandInitError::CannotOpenDisplay(display));
        }
        let display = display_ref;
        unsafe {
            DISPLAY.set(display); // Magic trick to avoid Rust's rules
        }

        let registry = unsafe { lib_wlc!(wl_display_get_registry)(display) };
        let mut listener = Box::new(WlRegistryListener::new());

        unsafe { lib_wlc!(wl_registry_add_listener)(registry, listener.as_mut() as *mut WlRegistryListener, null_mut()) }

        todo!()
    }
}
impl Drop for WaylandBackend {
    fn drop(&mut self) {
        unsafe { lib_wlc!(wl_display_disconnect)(self.display) };
    }
}

#[repr(C)]
pub struct WlDisplay([u8; 0]);

#[repr(C)]
pub struct WlRegistry([u8; 0]);

#[repr(C)]
pub struct WlCompositor([u8; 0]);

#[repr(C)]
struct WlInterface {
    name: *const c_char,
    ver: c_int,
    method_count: c_int,
    methods: *const WlMessage,
    event_count: c_int,
    events: *const WlMessage,
}

#[repr(C)]
pub struct WlRegistryListener {
    global_registry_handler: fn(*mut c_void, *mut WlRegistry, u32, *const c_char, u32),
    global_registry_remover: fn(*mut c_void, *mut WlRegistry, u32),
}
impl WlRegistryListener {
    fn global_registry_handler(
        _data: *mut c_void,
        registry: *mut WlRegistry,
        id: u32,
        interface: *const c_char,
        _version: u32,
    ) {
        let binding = unsafe { CStr::from_ptr(interface) }.to_owned();
        let interface = binding.to_str().unwrap();
        println!("Received object {interface} (id {id})");
        if interface == "wl_compositor" {
            println!("Obtained compositor!");
            unsafe {
                COMPOSITOR.set(lib_wlc!(wl_registry_bind)(
                    registry,
                    id,
                    &
                ) as *mut WlCompositor);
            }
        }
    }

    fn global_registry_remover(
        _data: *mut c_void,
        _registry: *mut WlRegistry,
        id: u32,
    ) {
        println!("Removed object {id}");
    }

    pub fn new() -> Self {
        Self {
            global_registry_handler: WlRegistryListener::global_registry_handler,
            global_registry_remover: WlRegistryListener::global_registry_remover,
        }
    }
}

library! {
    [LibWaylandClient <-> "wayland-client"];

    pub fn wl_display_connect(name: *const c_char) -> *mut WlDisplay;
    pub fn wl_display_disconnect(display: *mut WlDisplay);

    pub fn wl_display_get_registry(display: *mut WlDisplay) -> *mut WlRegistry;
    pub fn wl_display_dispatch(display: *mut WlDisplay);
    pub fn wl_display_roundtrip(display: *mut WlDisplay);

    pub fn wl_registry_add_listener(registry: *mut WlRegistry, listener: *mut WlRegistryListener, arg3: *mut c_void);
    pub fn wl_registry_bind(registry: *mut WlRegistry, id: u32, interface: *const *const c_char, ver: u32) -> *mut c_void;
}
