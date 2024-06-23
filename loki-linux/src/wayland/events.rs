#[allow(unused_imports)] // used in doc comments
use crate::wayland::{interfaces::all::*, wire::*};

pub trait Event: Sized {
    fn from_wire(decoder: &mut WireDecoder) -> Option<Self>;
}

events! {
/// Events for the [`WlDisplay`] interface.
pub enum WlDisplayEvent {
    type Interface = WlDisplay;

    /// There was a Wayland error. Stores the object ID that caused the error,
    /// the error code, and then an error message.
    Error(object_id: Id, code: u32, message: String) = 0,
    /// Acknowledges that an object was deleted. Stores the deleted object's ID.
    DeleteId(id: Id) = 1,
}

/// Events for the [`WlRegistry`] interface.
pub enum WlRegistryEvent {
    type Interface = WlRegistry;

    /// Announces that a global is available for the client to bind. Stores
    /// the global's name, the name of the global's interface, and the version
    /// of the global's interface. Globals can then be bound by the client with
    /// [`WlRegistry::bind`]
    Global(name: Name, interface: String, version: u32) = 0,
    /// Announces that a global object has been removed by the compositor. Stores
    /// the name of the removed global.
    GlobalRemove(name: Name) = 1
}

/// Events for the [`WlCallback`] interface.
pub enum WlCallbackEvent {
    type Interface = WlCallback;

    /// Announces that the callback has finished running. The meaning of the provided
    /// `u32` depends on the context the callback was used in.
    Done(callback_data: u32) = 0
}

pub enum WlShmEvent {
    type Interface = WlShm;

    Format(format: u32) = 0 // TODO: format is an enum
}

pub enum WlBufferEvent {
    type Interface = WlBuffer;

    Release = 0
}

pub enum WlSurfaceEvent {
    type Interface = WlSurface;

    Enter(output: WlOutput) = 0,
    Leave(output: WlOutput) = 1,
    PreferredBufferScale(factor: i32) = 2,
    PreferredBufferTransform(transform: u32) = 3, // TODO: transform is an enum
}

pub enum XdgWmBaseEvent {
    type Interface = XdgWmBase;

    Ping(serial: u32) = 0
}

pub enum XdgSurfaceEvent {
    type Interface = XdgSurface;

    Configure(serial: u32) = 0,
}

pub enum XdgToplevelEvent {
    type Interface = XdgToplevel;

    Configure(width: i32, height: i32, states: i32) = 0, // TODO: array type for states
    Close = 1,
    ConfigureBounds(width: i32, height: i32) = 2,
    WmCapabilities(capabilities: i32) = 3, // TODO: array type for wm_capabilities
}
}

#[macro_export]
macro_rules! events {
    ($($(#[doc = $doc:literal])* pub enum $name:ident {type Interface = $object_type:ident;$($(#[doc = $variant_doc:literal])* $variant:ident$(($($var:ident: $var_ty:ty),*))* = $val:literal),*$(,)*})*) => {
        /// Events that the Wayland compositor sends back to the client.
        pub enum WaylandEvent {
            $(
                $(#[doc = $doc])*
                $name($name)
            ),*
        }

        $(
        impl From<$name> for WaylandEvent {
            fn from(val: $name) -> Self {
                Self::$name(val)
            }
        }

        impl Event for $name {
            fn from_wire(decoder: &mut WireDecoder) -> Option<Self> {
                match decoder.opcode() {
                    $($val => {
                        Some(Self::$variant($object_type { id: decoder.object_id() }$(, $(decoder.decode::<$var_ty>()),*)*))
                    })*
                    _ => None
                }
            }
        }

        #[repr(u16)]
        $(#[doc = $doc])*
        pub enum $name {
            $(
                $(#[doc = $variant_doc])*
                $variant($object_type, $($($var_ty),*)*) = $val,
            )*
        }
        )*

        impl super::interfaces::Interface {
            /// Decodes a Wire event for this interface.
            pub fn decode_event(&self, decoder: &mut WireDecoder) -> Option<WaylandEvent> {
                Some(match self {
                    $(Self::$object_type => $name::from_wire(decoder)?.into(),)*
                    _ => {
                        println!("Loki-linux warning: Not decoding events for `{self}`");
                        return None;
                    }
                })
            }
        }
    };
}
pub use events;
