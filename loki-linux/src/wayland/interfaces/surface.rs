use crate::wayland::Object;

pub struct Surface {
    pub(crate) id: u32,
}
impl Object for Surface {
    fn id(&self) -> u32 {
        self.id
    }
    fn interface(&self) -> &'static str {
        "wl_surface"
    }
    fn version(&self) -> u32 {
        6
    }
}

pub struct SubSurface {}
