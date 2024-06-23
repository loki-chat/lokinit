use super::interface_prelude::*;

decl_interfaces! {
    WlDisplay {
        Events = WlDisplayEvent;
        Methods = WlDisplayMethod;
        GlobalSingleton = true;
    }

    WlRegistry {
        Events = WlRegistryEvent;
        Methods = WlRegistryMethod;
        GlobalSingleton = true;
    }

    WlCallback {
        Events = WlCallbackEvent;
        Methods = ();
    }

    WlCompositor {
        Events = ();
        Methods = WlCompositorMethod;
        GlobalSingleton = true;
    }

    WlShmPool {
        Events = ();
        Methods = WlShmPoolMethod;
    }

    WlShm {
        Events = WlShmEvent;
        Methods = WlShmMethod;
        GlobalSingleton = true;
    }

    WlBuffer {
        Events = WlBufferEvent;
        Methods = WlBufferMethod;
    }

    WlDataOffer {
        Events = (); // TODO
        Methods = (); // TODO
    }

    WlDataSource {
        Events = (); // TODO
        Methods = (); // TODO
    }

    WlDataDevice {
        Events = (); // TODO
        Methods = (); // TODO
    }

    WlDataDeviceManager {
        Events = ();
        Methods = (); // TODO
        GlobalSingleton = true;
    }

    WlSurface {
        Events = WlSurfaceEvent;
        Methods = WlSurfaceMethod;
    }

    WlSeat {
        Events = (); // TODO
        Methods = (); // TODO
        GlobalSingleton = true;
    }

    WlPointer {
        Events = (); // TODO
        Methods = (); // TODO
    }

    WlKeyboard {
        Events = (); // TODO
        Methods = (); // TODO
    }

    WlTouch {
        Events = (); // TODO
        Methods = (); // TODO
    }

    WlOutput {
        Events = (); // TODO
        Methods = (); // TODO
        GlobalSingleton = true;
    }

    WlRegion {
        Events = ();
        Methods = (); // TODO
    }

    WlSubcompositor {
        Events = ();
        Methods = (); // TODO
        GlobalSingleton = true;
    }

    WlSubsurface {
        Events = ();
        Methods = (); // TODO
    }
}

impl WlDisplay {
    pub const fn global() -> Self {
        Self { id: Id { raw: 1 } }
    }
}

impl WlRegistry {
    pub const fn global() -> Self {
        Self { id: Id { raw: 2 } }
    }
}

impl WlCallback {
    pub fn new(client: &mut WaylandClient, cb: Box<dyn Fn(u32)>) -> Self {
        let id = client.next_object_id();
        client.callbacks.insert(id, cb);
        client.objects[id.raw as usize] = Some(Interface::WlCallback);

        Self { id }
    }
    pub fn new_empty(client: &mut WaylandClient) -> Self {
        let id = client.next_object_id();
        client.objects[id.raw as usize] = Some(Interface::WlCallback);

        Self { id }
    }
}
impl WlCompositor {
    pub fn create_surface(&self, client: &mut WaylandClient) -> WlSurface {
        let id = client.next_object_id();
        client.call_method(self, WlCompositorMethod::CreateSurface(id));
        client.objects[id.raw as usize] = Some(Interface::WlSurface);

        WlSurface { id }
    }
    pub fn create_region(&self, client: &mut WaylandClient) -> WlRegion {
        let id = client.next_object_id();
        client.call_method(self, WlCompositorMethod::CreateRegion(id));
        client.objects[id.raw as usize] = Some(Interface::WlRegion);

        WlRegion { id }
    }
}
impl WlShm {
    pub fn create_pool(&self, client: &mut WaylandClient, fd: Fd, size: i32) -> WlShmPool {
        let id = client.next_object_id();
        client.call_method(self, WlShmMethod::CreatePool(id, fd, size));
        client.objects[id.raw as usize] = Some(Interface::WlShmPool);

        WlShmPool { id }
    }
}
impl WlShmPool {
    pub fn create_buffer(
        &self,
        client: &mut WaylandClient,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    ) -> WlBuffer {
        // TODO: format is an enum
        let id = client.next_object_id();
        client.call_method(
            self,
            WlShmPoolMethod::CreateBuffer(id, offset, width, height, stride, format),
        );
        client.objects[id.raw as usize] = Some(Interface::WlBuffer);

        WlBuffer { id }
    }
}
