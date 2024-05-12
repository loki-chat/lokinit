use {
    super::WaylandState,
    loki_linux::wayland::interfaces::{
        compositor::Compositor,
        registry::{RegistryListener, WaylandRegistry},
        xdg::XdgWmBase,
    },
    std::{cell::RefCell, rc::Rc},
};

pub struct RegistryEventListener {
    pub state: Rc<RefCell<WaylandState>>,
}
impl RegistryListener for RegistryEventListener {
    fn global(&self, name: u32, interface: &str, _version: u32) {
        // TODO: Check version?
        match interface {
            "wl_compositor" => {
                let compositor = WaylandRegistry.bind_new::<Compositor>(name);
                self.state.borrow_mut().compositor = Some(compositor);
            }
            "xdg_wm_base" => {
                let xdg_wm_base = WaylandRegistry.bind_new::<XdgWmBase>(name);
                self.state.borrow_mut().xdg_wm_base = Some(xdg_wm_base);
            }
            _ => {}
        }
    }

    fn global_remove(&self, _name: u32) {}
}
