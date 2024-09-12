use {
    super::WaylandBackend,
    crate::{
        event::{Event, EventKind},
        prelude::WindowHandle,
    },
    loki_linux::wayland::{events::*, interfaces::all::*, methods::*},
    std::time::Duration,
};

impl WaylandBackend {
    pub fn handle_event(&mut self, event: WaylandEvent) -> Option<Event> {
        match event {
            WaylandEvent::WlDisplayEvent(display_event) => match display_event {
                WlDisplayEvent::Error(_, obj, error_code, error) => {
                    let interface = self
                        .client
                        .object_interface(obj)
                        .expect("Wayland compositor sent an invalid object ID");
                    panic!(
                        "\
                        Lokinit: Critical wayland error!\n\
                        Object: #{obj} (interface: `{interface}`)\n\
                        Wayland Error Code: {error_code}\n\
                        Wayland Error Message: `{error}`\
                        ",
                    )
                }
                _ => None,
            },

            WaylandEvent::WlRegistryEvent(registry_event) => match registry_event {
                WlRegistryEvent::Global(_, name, interface, _version) => {
                    println!("Trying to register object {}", interface.as_str());
                    match interface.as_str() {
                        "wl_compositor" => {
                            self.client.bind_global::<WlCompositor>(name);
                        }
                        "xdg_wm_base" => {
                            self.client.bind_global::<XdgWmBase>(name);
                        }
                        "wl_shm" => {
                            self.client.bind_global::<WlShm>(name);
                        }
                        _ => {}
                    }
                    None
                }
                _ => None,
            },

            WaylandEvent::XdgWmBaseEvent(XdgWmBaseEvent::Ping(xdg_wm_base, serial)) => {
                self.client
                    .call_method(&xdg_wm_base, XdgWmBaseMethod::Pong(serial));
                None
            }

            WaylandEvent::XdgSurfaceEvent(XdgSurfaceEvent::Configure(xdg_surface, serial)) => {
                let window = self.windows[*self.object_to_window_map.get(&xdg_surface.id).unwrap()]
                    .as_ref()
                    .unwrap();
                self.client.call_method(
                    &window.wl_surface,
                    WlSurfaceMethod::Attach(Some(window.buffer.wl_buffer()), 0, 0),
                );
                self.client
                    .call_method(&xdg_surface, XdgSurfaceMethod::AckConfigure(serial));
                self.client
                    .call_method(&window.wl_surface, WlSurfaceMethod::Commit);
                None
            }

            WaylandEvent::XdgToplevelEvent(toplevel_event) => match toplevel_event {
                XdgToplevelEvent::ConfigureBounds(xdg_toplevel, width, height) => {
                    let window_id = *self.object_to_window_map.get(&xdg_toplevel.id).unwrap();
                    Some(Event {
                        time: Duration::ZERO,
                        window: WindowHandle(window_id),
                        kind: EventKind::Resized(width as _, height as _),
                    })
                }
                _ => None,
            },

            _ => None,
        }
    }
}
