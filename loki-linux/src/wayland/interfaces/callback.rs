use {
    crate::wayland::{wire::MessageHeader, Listener, Object, WaylandStorage},
    std::{any::Any, rc::Rc},
};

/// Stores a function that will get called with data later.
pub struct Callback {
    pub(crate) id: u32,
}
impl Callback {
    pub fn new(func: impl Fn(u32) + 'static) -> Self {
        WaylandStorage::with_global(|global| {
            let id = global.alloc_object_id();
            global.objects.insert(id, Some(Box::new(Self { id })));
            global
                .listeners
                .insert(id, Some(Rc::new(CallbackListener(func))));

            Self { id }
        })
    }
}
impl Object for Callback {
    fn id(&self) -> u32 {
        self.id
    }
    fn interface(&self) -> &'static str {
        "wl_callback"
    }
    fn version(&self) -> u32 {
        1
    }
}

pub struct CallbackListener<F: Fn(u32)>(F);
impl<F: Fn(u32)> Listener for CallbackListener<F> {
    fn parse(&self, header: MessageHeader, data: &[u32]) {
        if header.opcode == 0 {
            (self.0)(data[0])
        }
    }
}
