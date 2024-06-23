use crate::wayland::{interfaces::all::*, wire::*};

/// Implemented for all enums that represent Wayland object methods.
pub trait ObjectMethods: WriteWire {
    /// Get the opcode of this method.
    fn opcode(&self) -> u16;
}

methods! {
/// Methods for the [`WlDisplay`] type.
pub enum WlDisplayMethod {
    /// After the compositor has finished processing all messages sent to it (before this method
    /// was called), it calls the provided callback.
    Sync(callback: WlCallback) = 0,
    /// Creates a new global [`WlRegistry`] singleton with the provided object ID. Loki
    /// automatically calls this at startup with ID 2 - use [`WlRegistry::get_global`] to
    /// access it.
    GetRegistry(registry: Id) = 1,
}

pub enum WlRegistryMethod {
    /// Binds a global object with the provided name to the provided object ID. The first
    /// argument is the name, the second is the object ID.
    Bind(name: Name, id: NewId) = 0,
}

pub enum WlCompositorMethod {
    /// Create a [`WlSurface`] with the given ID.
    CreateSurface(id: Id) = 0,
    /// Create a [`WlRegion`] with the given ID.
    CreateRegion(id: Id) = 1
}

pub enum WlShmPoolMethod {
    CreateBuffer(id: Id, offset: i32, width: i32, height: i32, stride: i32, format: u32) = 0, // TODO: format is an enum
    Destroy = 1,
    Resize(size: i32) = 2,
}

pub enum WlShmMethod {
    CreatePool(id: Id, fd: Fd, size: i32) = 0,
}

pub enum WlBufferMethod {
    Destroy = 0
}

pub enum WlSurfaceMethod {
    Destroy = 0,
    Attach(buffer: Option<WlBuffer>, x: i32, y: i32) = 1,
    Damage(x: i32, y: i32, width: i32, height: i32) = 2,
    Frame(callback: WlCallback) = 3,
    SetOpaqueRegion(region: Option<WlRegion>) = 4,
    SetInputRegion(region: Option<WlRegion>) = 5,
    Commit = 6,
    SetBufferTransform(transform: i32) = 7, // TODO: transform is an enum
    SetBufferScale(scale: i32) = 8,
    DamageBuffer(x: i32, y: i32, width: i32, height: i32) = 9,
    Offset(x: i32, y: i32) = 10
}

pub enum XdgWmBaseMethod {
    Destroy = 0,
    CreatePositioner(id: Id) = 1,
    GetXdgSurface(id: Id, surface: WlSurface) = 2,
    Pong(serial: u32) = 3
}

pub enum XdgSurfaceMethod {
    Destroy = 0,
    GetToplevel(id: Id) = 1,
    GetPopup(id: Id, parent: Option<XdgSurface>, positioner: XdgPositioner) = 2,
    SetWindowGeometry(x: i32, y: i32, width: i32, height: i32) = 3,
    AckConfigure(serial: u32) = 4,
}

pub enum XdgToplevelMethod {
    Destroy = 0,
    SetParent(parent: Option<XdgToplevel>) = 1,
    SetTitle(title: String) = 2,
    SetAppId(app_id: String) = 3,
    ShowWindowMenu(seat: Id, serial: u32, x: i32, y: i32) = 4,
    Move(seat: Id, serial: u32) = 5,
    Resize(seat: Id, serial: u32, edges: u32) = 6, // TODO: edges is an enum variant
    SetMaxSize(width: i32, height: i32) = 7,
    SetMinSize(width: i32, height: i32) = 8,
    SetMaximized = 9,
    UnsetMaximized = 10,
    SetFullscreen(output: Option<WlOutput>) = 11,
    UnsetFullscreen = 12,
    SetMinimized = 13,
}
}

#[macro_export]
macro_rules! methods {
    ($($(#[doc = $doc:literal])* pub enum $name:ident {$($(#[doc = $variant_doc:literal])* $variant:ident$(($($var:ident: $var_ty:ty),*))* = $val:literal),*$(,)*})*) => {
        /// Object methods that the Wayland client sends to the compositor.
        pub enum WaylandMethod {
            $(
                $(#[doc = $doc])*
                $name($name)
            ),*
        }
        #[allow(non_snake_case)]
        impl WriteWire for WaylandMethod {
            fn write_wire(&self, encoder: &mut WireEncoder) {
                match self {
                    $(
                        Self::$name($name) => $name.write_wire(encoder),
                    )*
                }
            }
        }
        #[allow(non_snake_case)]
        impl ObjectMethods for WaylandMethod {
            fn opcode(&self) -> u16 {
                match self {
                    $(
                        Self::$name($name) => $name.opcode(),
                    )*
                }
            }
        }

        $(
        impl From<$name> for WaylandMethod {
            fn from(val: $name) -> Self {
                Self::$name(val)
            }
        }

        impl ObjectMethods for $name {
            fn opcode(&self) -> u16 {
                // https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting
                unsafe { *(self as *const $name as *const u16) }
            }
        }

        #[allow(non_snake_case, unused_variables)]
        impl WriteWire for $name {
            fn write_wire(&self, encoder: &mut WireEncoder) {
                match self {
                    $(
                        Self::$variant$(($($var),*))* => {
                            $($($var.write_wire(encoder);)*)*
                        }
                    )*
                }
            }
        }

        #[repr(u16)]
        $(#[doc = $doc])*
        pub enum $name {
            $(
                $(#[doc = $variant_doc])*
                $variant$(($($var_ty),*))* = $val,
            )*
        }
        )*
    };
}
pub use methods;
