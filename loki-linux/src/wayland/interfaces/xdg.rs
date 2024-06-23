use super::interface_prelude::*;

decl_interfaces! {
    XdgWmBase {
        Events = XdgWmBaseEvent;
        Methods = XdgWmBaseMethod;
        GlobalSingleton = true;
    }

    XdgPositioner {
        Events = ();
        Methods = (); // TODO
    }

    XdgSurface {
        Events = XdgSurfaceEvent;
        Methods = XdgSurfaceMethod;
    }

    XdgPopup {
        Events = (); // TODO
        Methods = (); // TODO
    }

    XdgToplevel {
        Events = XdgToplevelEvent;
        Methods = XdgToplevelMethod;
    }
}

impl XdgWmBase {
    pub fn get_xdg_surface(&self, client: &mut WaylandClient, surface: WlSurface) -> XdgSurface {
        let id = client.next_object_id();
        client.call_method(self, XdgWmBaseMethod::GetXdgSurface(id, surface));
        client.objects[id.raw as usize] = Some(Interface::XdgSurface);

        XdgSurface { id }
    }
}
impl XdgSurface {
    pub fn get_toplevel(&self, client: &mut WaylandClient) -> XdgToplevel {
        let id = client.next_object_id();
        client.call_method(self, XdgSurfaceMethod::GetToplevel(id));
        client.objects[id.raw as usize] = Some(Interface::XdgToplevel);

        XdgToplevel { id }
    }
    pub fn get_popup(
        &self,
        client: &mut WaylandClient,
        parent: Option<XdgSurface>,
        positioner: XdgPositioner,
    ) -> XdgPopup {
        let id = client.next_object_id();
        client.call_method(self, XdgSurfaceMethod::GetPopup(id, parent, positioner));
        client.objects[id.raw as usize] = Some(Interface::XdgPopup);

        XdgPopup { id }
    }
}
