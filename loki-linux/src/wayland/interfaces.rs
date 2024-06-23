pub mod core;
pub mod xdg;

pub mod all {
    pub use super::{core::*, xdg::*, Interface};
}

mod interface_prelude {
    pub use super::decl_interfaces;
    pub use crate::wayland::{
        events::*,
        interfaces::all::*,
        methods::*,
        wire::{Fd, Id},
        GlobalSingleton, Object, WaylandClient,
    };
}

#[macro_export]
macro_rules! decl_interfaces {
    ($(
        $ty:ident {
            Events = $events:ty;
            Methods = $methods:ty;
            $(GlobalSingleton = $unused:ident;)*
        }
    )*) => {
        $(
        #[derive(Clone, Copy, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $ty {
            pub id: Id,
        }
        impl Object for $ty {
            const INTERFACE: Interface = Interface::$ty;
            type Events = $events;
            type Methods = $methods;

            fn id(&self) -> Id {
                self.id
            }
            fn new_with_id(id: Id) -> Self {
                Self { id }
            }
        }
        $(
            impl GlobalSingleton for $ty {
                const UNUSED: bool = $unused;
            }
        )*
        )*
    };
}
pub use decl_interfaces;

/// All of the Wayland interfaces used by Loki.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interface {
    WlDisplay,
    WlRegistry,
    WlCallback,
    WlCompositor,
    WlShmPool,
    WlShm,
    WlBuffer,
    WlDataOffer,
    WlDataSource,
    WlDataDevice,
    WlDataDeviceManager,
    WlSurface,
    WlSeat,
    WlPointer,
    WlKeyboard,
    WlTouch,
    WlOutput,
    WlRegion,
    WlSubcompositor,
    WlSubsurface,
    XdgWmBase,
    XdgPositioner,
    XdgSurface,
    XdgToplevel,
    XdgPopup,
}
impl Interface {
    /// The version of this interface that Loki supports.
    pub const fn version_number(&self) -> u32 {
        match *self {
            Self::WlDisplay => 1,
            Self::WlRegistry => 1,
            Self::WlCallback => 1,
            Self::WlCompositor => 6,
            Self::WlShmPool => 1,
            Self::WlShm => 1,
            Self::WlBuffer => 1,
            Self::WlDataOffer => 3,
            Self::WlDataSource => 3,
            Self::WlDataDevice => 3,
            Self::WlDataDeviceManager => 3,
            Self::WlSurface => 6,
            Self::WlSeat => 9,
            Self::WlPointer => 9,
            Self::WlKeyboard => 9,
            Self::WlTouch => 9,
            Self::WlOutput => 4,
            Self::WlRegion => 1,
            Self::WlSubcompositor => 1,
            Self::WlSubsurface => 1,
            Self::XdgWmBase => 6,
            Self::XdgPositioner => 6,
            Self::XdgSurface => 6,
            Self::XdgToplevel => 6,
            Self::XdgPopup => 6,
        }
    }
}
impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::WlDisplay => "wl_display",
            Self::WlRegistry => "wl_registry",
            Self::WlCallback => "wl_callback",
            Self::WlCompositor => "wl_compositor",
            Self::WlShmPool => "wl_shm_pool",
            Self::WlShm => "wl_shm",
            Self::WlBuffer => "wl_buffer",
            Self::WlDataOffer => "wl_data_offer",
            Self::WlDataSource => "wl_data_source",
            Self::WlDataDevice => "wl_data_device",
            Self::WlDataDeviceManager => "wl_data_device_manager",
            Self::WlSurface => "wl_surface",
            Self::WlSeat => "wl_seat",
            Self::WlPointer => "wl_pointer",
            Self::WlKeyboard => "wl_keyboard",
            Self::WlTouch => "wl_touch",
            Self::WlOutput => "wl_output",
            Self::WlRegion => "wl_region",
            Self::WlSubcompositor => "wl_subcompositor",
            Self::WlSubsurface => "wl_subsurface",
            Self::XdgWmBase => "xdg_wm_base",
            Self::XdgPositioner => "xdg_positioner",
            Self::XdgSurface => "xdg_surface",
            Self::XdgToplevel => "xdg_toplevel",
            Self::XdgPopup => "xdg_popup",
        };

        write!(f, "{name}")
    }
}
