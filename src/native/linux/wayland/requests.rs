use super::ffi::WL_DISPLAY_GET_REGISTRY;
use super::ffi::{WlDisplay, WlRegistry, LibWaylandClient, WlProxy};

macro_rules! wl_request_constructor {
    ($wl:expr, $instance:expr, $request_name:expr, $interface:expr) => {
        wl_request_constructor!($wl, $instance, $request_name, $interface, ())
    };

    ($wl:expr, $instance:expr, $request_name:expr, $interface:expr, $($arg:expr),*) => {{
        let id: *mut WlProxy;

        id = ($wl.wl_proxy_marshal_constructor)(
            $instance as _,
            $request_name,
            $interface as _,
            std::ptr::null_mut::<std::ffi::c_void>(),
            $($arg,)*
        );

        id as *mut _
    }};
}

macro_rules! wl_request {
    ($wl:expr, $instance:expr, $request_name:expr) => {
        wl_request!($wl, $instance, $request_name, ())
    };

    ($wl:expr, $instance:expr, $request_name:expr, $($arg:expr),*) => {{
        ($wl.wl_proxy_marshal)(
            $instance as _,
            $request_name,
            $($arg,)*
        )
    }};
}

impl LibWaylandClient {
    pub unsafe fn wl_display_get_registry(&self, display: *mut WlDisplay) -> *mut WlRegistry {
        wl_request_constructor!(
            self,
            display,
            WL_DISPLAY_GET_REGISTRY,
            self.wl_registry_interface
        )
    }
}
