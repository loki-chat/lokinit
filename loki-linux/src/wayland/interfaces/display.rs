use {
    crate::wayland::{
        interfaces::{callback::Callback, registry::WaylandRegistry},
        wire::{MessageHeader, WireType},
        Listener, Object, WaylandClient, WaylandStorage,
    },
    std::{any::Any, rc::Rc},
};

/// The global display singleton.
pub struct WaylandDisplay;
impl WaylandDisplay {
    /// Wayland defines the display as always being object 1.
    const OBJECT_ID: u32 = 1;

    /// Asks the server to process all queued events. After they're processed, the server will fire the
    /// `done` event on the provided callback with the event serial number.
    pub fn sync(&self, callback: Callback) {
        WaylandStorage::with_global(|global| {
            global.send_message(self, 0, &[&callback.id()]);
        });
    }

    /// Get the global [`Registry`] object. Note that registry is a global singleton, so this doesn't do anything,
    /// it's just here to match the normal Wayland API.
    pub fn get_registry(&self) -> WaylandRegistry {
        WaylandRegistry
    }

    /// Set a listener to receive callbacks for events.
    pub fn set_listener(&self, listener: impl DisplayListener + 'static) {
        WaylandStorage::with_global(move |global| {
            global.listeners.insert(
                Self::OBJECT_ID,
                Some(Rc::new(DisplayListenerStore(listener))),
            );
        });
    }

    /// Sends the `get_registry` message. Loki automatically calls this at startup to allocate the registry to object 2.
    pub fn _msg_get_registry(&self, object_id: u32) {
        WaylandStorage::with_global(|global| {
            global.send_message(self, 1, &[&object_id]);
        });
    }
}

/// Callbacks for the events [`Display`] receives.
pub trait DisplayListener {
    /// A fatal error occurred. The object is what caused the error; the code depends on the object's type;
    /// the message is a brief description of the error.
    fn error(&self, object: &dyn Object, code: u32, message: &str);

    /// The server sends this event to acknowledge that an object has been deleted. Loki will automatically
    /// remove the object for you after calling this event.
    ///
    /// Wayland refers to this event as `delete_id`. Because Loki handles object management automatically
    /// and can pass the object instead of the object's ID, it calls it `delete_object`.
    fn delete_object(&self, object: &dyn Object);
}

#[repr(u32)]
pub enum DisplayError {
    InvalidObject = 0,
    InvalidMethod = 1,
    NoMemory = 2,
    Implementation = 3,
}

impl Object for WaylandDisplay {
    fn id(&self) -> u32 {
        Self::OBJECT_ID
    }
    fn interface(&self) -> &'static str {
        "wl_display"
    }
    fn version(&self) -> u32 {
        1
    }
}

struct DisplayListenerStore<L: DisplayListener>(L);
impl<L: DisplayListener> Listener for DisplayListenerStore<L> {
    fn parse(&self, header: MessageHeader, data: &[u32]) {
        match header.opcode {
            0 => {
                let id = data[0];
                WaylandStorage::with_global(|global| {
                    let object = global.objects.get(&id).unwrap().as_ref().unwrap().as_ref();
                    self.0
                        .error(object, data[1], <&str>::from_wire(&data[2..]).0)
                });
            }
            1 => {
                let id = data[0];
                WaylandStorage::with_global(|global| {
                    let object = global.objects.get_mut(&id).unwrap();
                    self.0.delete_object(object.as_ref().unwrap().as_ref());
                    *object = None;
                });
            }
            _ => {}
        }
    }
}
