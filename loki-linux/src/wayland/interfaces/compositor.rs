use {
    super::{region::Region, surface::Surface},
    crate::wayland::{GlobalObject, Object, WaylandStorage},
};

/// The compositor is a global singleton. It renders surfaces.
pub struct Compositor {
    id: u32,
}
impl Compositor {
    /// Creates a [`Surface`].
    pub fn create_surface(&self) -> Surface {
        WaylandStorage::with_global(|global| {
            let id = global.alloc_object_id();
            global.send_message(self, 0, &[&id]);
            Surface { id }
        })
    }

    /// Creates a [`Region`].
    pub fn create_region(&self) -> Region {
        todo!()
    }
}
impl Object for Compositor {
    fn id(&self) -> u32 {
        self.id
    }
    fn interface(&self) -> &'static str {
        "wl_compositor"
    }
    fn version(&self) -> u32 {
        6
    }
}
impl GlobalObject for Compositor {
    fn new_with_id(id: u32) -> Self {
        Self { id }
    }
}

pub struct SubCompositor {}
