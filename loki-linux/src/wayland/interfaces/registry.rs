use {
    crate::wayland::{
        wire::{MessageHeader, NewId, WireType},
        GlobalObject, Listener, Object, WaylandClient, WaylandStorage,
    },
    std::{any::Any, convert::Infallible, rc::Rc},
};

/// The global registry singleton.
pub struct WaylandRegistry;
impl WaylandRegistry {
    /// Loki always allocates the registry to object 2.
    const OBJECT_ID: u32 = 2;

    /// Binds an object to the compositor.
    pub fn bind(&self, name: u32, object: &impl Object) {
        WaylandStorage::with_global(|global| {
            global.name_to_id_map.insert(name, object.id());
            global.send_message(self, 0, &[&name, &NewId(object)]);
        });
    }

    /// Creates and binds an object to the compositor.
    pub fn bind_new<O: GlobalObject>(&self, name: u32) -> O {
        let id = WaylandStorage::with_global(|global| {
            let id = global.alloc_object_id();
            global
                .objects
                .insert(id, Some(Box::new(O::new_with_id(id))));
            id
        });

        let obj = O::new_with_id(id);
        self.bind(name, &obj);

        obj
    }

    /// Set a listener to receive callbacks for events.
    pub fn set_listener(&self, listener: impl RegistryListener + 'static) {
        WaylandStorage::with_global(|global| {
            global.listeners.insert(
                Self::OBJECT_ID,
                Some(Rc::new(RegistryListenerStore(listener))),
            );
        });
    }
}

pub trait RegistryListener {
    /// Announces a object is available.
    fn global(&self, name: u32, interface: &str, version: u32);
    /// Announces that a global object was removed. After calling this function, Loki will automatically remove
    /// the object's listener (if it had one) from the global [`WaylandStorage`].
    fn global_remove(&self, name: u32);
}

impl Object for WaylandRegistry {
    fn id(&self) -> u32 {
        Self::OBJECT_ID
    }
    fn interface(&self) -> &'static str {
        "wl_registry"
    }
    fn version(&self) -> u32 {
        1
    }
}

struct RegistryListenerStore<L: RegistryListener>(L);
impl<L: RegistryListener> Listener for RegistryListenerStore<L> {
    fn parse(&self, header: MessageHeader, data: &[u32]) {
        match header.opcode {
            0 => {
                let name = u32::from_wire(data);
                let data = &data[name.1..];
                let interface = <&str>::from_wire(data);
                let data = &data[interface.1..];
                let version = u32::from_wire(data);

                self.0.global(name.0, interface.0, version.0);
            }
            1 => {
                let name = data[0];
                self.0.global_remove(name);
                WaylandStorage::with_global(|global| {
                    let id = global.name_to_id_map.get(&name);
                    if let Some(id) = id {
                        global.listeners.insert(*id, None);
                        global.objects.insert(*id, None);
                    }
                });
            }
            _ => {}
        }
    }
}
