use {
    super::surface::Surface,
    crate::wayland::{GlobalObject, Object, WaylandStorage},
};

/// A global object that allows creating windows for desktop environments.
pub struct XdgWmBase {
    id: u32,
}
impl XdgWmBase {
    pub fn destroy(self) {
        todo!()
    }

    pub fn create_positioner(&self) {
        todo!()
    }

    /// Convert a [`Surface`] into an [`XdgSurface`].
    pub fn get_xdg_surface(&self, surface: &Surface) -> XdgSurface {
        WaylandStorage::with_global(|global| {
            let id = global.alloc_object_id();
            global.send_message(self, 2, &[&id, surface]);
            global.objects.insert(id, Some(Box::new(XdgSurface { id })));

            XdgSurface { id }
        })
    }

    /// Respond to a [`XdgWmBaseListener::ping`] event.
    pub fn pong(&self, serial: u32) {
        WaylandStorage::with_global(|global| {
            global.send_message(self, 3, &[&serial]);
        });
    }
}
impl Object for XdgWmBase {
    fn id(&self) -> u32 {
        self.id
    }
    fn interface(&self) -> &'static str {
        "xdg_wm_base"
    }
    fn version(&self) -> u32 {
        6
    }
}
impl GlobalObject for XdgWmBase {
    fn new_with_id(id: u32) -> Self {
        Self { id }
    }
}

/// Events from the [`XdgWmBase`] interface.
pub trait XdgWmBaseListener {
    /// The compositor fires this event every now and then to check that the client
    /// isn't frozen. You must respond to this event by calling [`XdgWmBase::pong`]
    /// with the given value for `serial`.
    fn ping(&self, serial: u32);
}

/// Represents a desktop window.
pub struct XdgSurface {
    id: u32,
}
impl Object for XdgSurface {
    fn id(&self) -> u32 {
        self.id
    }
    fn interface(&self) -> &'static str {
        "xdg_surface"
    }
    fn version(&self) -> u32 {
        6
    }
}
