use std::ffi::{c_char, CStr};
use std::fmt;
use std::os::raw::c_void;
use std::rc::Rc;
use std::{ffi::CString, ptr::NonNull};

use loki_linux::wayland::interfaces::wl_compositor::WlCompositor;
use loki_linux::wayland::interfaces::wl_display::WlDisplay;
use loki_linux::wayland::interfaces::wl_registry::WlRegistry;
use loki_linux::wayland::{LibWaylandClient, WlRegistryListener};
use loki_linux::LoadingError;

mod requests;

#[derive(Debug)]
pub enum WaylandInitError {
    NoLibWayland(LoadingError),
    NoDisplaySet,
    InvalidDisplayFormat,
    CannotOpenDisplay(String),
}

impl std::error::Error for WaylandInitError {}

impl fmt::Display for WaylandInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoLibWayland(err) => write!(f, "Cannot find libwayland-client: {:?}", err),
            Self::NoDisplaySet => write!(f, "WAYLAND_DISPLAY environment variable is not set"),
            Self::InvalidDisplayFormat => write!(f, "WAYLAND_DISPLAY is not a valid C string"),
            Self::CannotOpenDisplay(display) => {
                write!(f, "Cannot find Wayland display: {:?}", display)
            }
        }
    }
}

impl From<LoadingError> for WaylandInitError {
    fn from(value: LoadingError) -> Self {
        Self::NoLibWayland(value)
    }
}

pub struct WaylandBackend {
    wl: Rc<LibWaylandClient>,
    display: NonNull<WlDisplay>,
    listener: Box<WlRegistryListener>,
}

impl WaylandBackend {
    pub fn init() -> Result<Self, WaylandInitError> {
        unsafe {
            let display_env =
                std::env::var("WAYLAND_DISPLAY").map_err(|_| WaylandInitError::NoDisplaySet)?;

            let display_cstr = CString::new(display_env.clone())
                .map_err(|_| WaylandInitError::InvalidDisplayFormat)?;

            let wl = Rc::new(LibWaylandClient::new()?);

            let display = (wl.wl_display_connect)(display_cstr.as_ptr());
            let display =
                NonNull::new(display).ok_or(WaylandInitError::CannotOpenDisplay(display_env))?;

            let mut listener = Box::new(WlRegistryListener {
                global_registry_handler,
                global_registry_remover,
            });

            let registry = wl.wl_display_get_registry(display.as_ptr());
            let mut registry_state = Box::new(RegistryState::new(wl.clone()));
            wl.wl_registry_add_listener(
                registry,
                listener.as_mut() as *mut _ as _,
                registry_state.as_mut() as *mut _ as _,
            );

            (wl.wl_display_roundtrip)(display.as_ptr());

            Ok(Self {
                wl,
                display,
                listener,
            })
        }
    }
}

impl Drop for WaylandBackend {
    fn drop(&mut self) {
        unsafe { (self.wl.wl_display_disconnect)(self.display.as_ptr()) };
    }
}

pub struct RegistryState {
    pub wl: Rc<LibWaylandClient>,
    pub compositor: *mut WlCompositor,
}

impl RegistryState {
    fn new(wl: Rc<LibWaylandClient>) -> Self {
        Self {
            wl,
            compositor: std::ptr::null_mut(),
        }
    }
}

unsafe extern "C" fn global_registry_handler(
    data: *mut c_void,
    registry: *mut WlRegistry,
    id: u32,
    interface_name: *const c_char,
    version: u32,
) {
    let Some(data) = data.cast::<RegistryState>().as_mut() else {
        // No data? ',:v
        return;
    };

    let interface_name = CStr::from_ptr(interface_name).to_str().unwrap().to_owned();
    if interface_name == "wl_compositor" {
        let interface = data.wl.wl_registry_interface;
        data.compositor = data.wl.wl_registry_bind(registry, id, interface, version) as _;
        todo!()
    }
}

unsafe extern "C" fn global_registry_remover(
    _data: *mut c_void,
    _registry: *mut WlRegistry,
    id: u32,
) {
    println!("Removing object {id} (not really)");
}
