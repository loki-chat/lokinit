pub mod enums;
pub mod events;
pub mod interfaces;
pub mod methods;
pub mod socket;
pub mod wire;

use {
    self::{
        events::*,
        interfaces::{all::*, Interface},
        wire::*,
    },
    crate::hashnt::Hashnt,
    methods::*,
    std::{
        borrow::Borrow,
        collections::HashMap,
        env,
        fs::OpenOptions,
        io::Read,
        os::{
            fd::FromRawFd,
            unix::{fs::OpenOptionsExt, net::UnixStream},
        },
        path::PathBuf,
    },
    wire::{WireEncoder, WriteWire},
};

pub struct WaylandClient {
    /// The socket we use to communicate with the Wayland compositor.
    pub socket: UnixStream,
    /// Stores the interface of every object that currently exists.
    pub objects: Vec<Option<Interface>>,
    /// Stores global singletons by their interface type.
    pub globals: HashMap<Interface, Id, Hashnt>,
    /// Maps object names to object IDs. The keys are the object names, the values are the object IDs.
    pub names: HashMap<Name, Id, Hashnt>,
    /// Stores callback functions by their object IDs.
    pub callbacks: HashMap<Id, Box<dyn Fn(u32)>, Hashnt>,
    /// A reusable buffer for reading messages from the compositor. This avoids needless reallocations
    /// for every message read.
    msg_buffer: Vec<u8>,
}
impl WaylandClient {
    /// Tries to connect to a Wayland compositor. If successful, creates and returns
    /// a [`WaylandClient`]. This will also automatically create a [`WlRegistry`] singleton
    /// with an object ID of 2, and then perform a [`Self::roundtrip`].
    pub fn new() -> Option<Self> {
        let socket = Self::find_compositor()?;

        let mut this = Self {
            socket,
            objects: Vec::with_capacity(3),
            globals: HashMap::with_capacity_and_hasher(2, Hashnt),
            names: HashMap::with_capacity_and_hasher(2, Hashnt),
            callbacks: HashMap::default(),
            msg_buffer: Vec::default(),
        };

        // Object 0 is always null/none
        this.objects.push(None);
        // Object 1 is always the global display singleton
        this.objects.push(Some(Interface::WlDisplay));
        this.globals.insert(Interface::WlDisplay, Id { raw: 1 });
        // Object 2 is set as the global registry singleton by Loki
        this.call_method(
            &WlDisplay::global(),
            WlDisplayMethod::GetRegistry(Id { raw: 2 }),
        );
        this.objects.push(Some(Interface::WlRegistry));
        this.globals.insert(Interface::WlRegistry, Id { raw: 2 });

        Some(this)
    }

    /// Attempts to locate the Wayland compositor. This method is taken from:
    /// https://wayland-book.com/protocol-design/wire-protocol.html#transports
    pub fn find_compositor() -> Option<UnixStream> {
        if let Ok(socket) = env::var("WAYLAND_SOCKET") {
            if let Ok(socket) = socket.parse::<i32>() {
                return Some(unsafe { UnixStream::from_raw_fd(socket) });
            }
        }

        if let Ok(runtime_dir) = env::var("XDG_RUNTIME_DIR") {
            if let Ok(display) = env::var("WAYLAND_DISPLAY") {
                let path = PathBuf::from(runtime_dir);
                return UnixStream::connect(path.join(display)).ok();
            } else {
                return UnixStream::connect(runtime_dir + "wayland-0").ok();
            }
        }

        None
    }

    /// Gets and returns a [`WaylandEvent`], if one is sent from the compositor. After
    /// processing this event, you should call [`WaylandClient::process_event`], because
    /// there's a few events the client needs to handle internally as well. See
    /// [`WaylandClient::process_event`]'s docs for more info.
    ///
    /// The following is a list of the events this method will automatically handle before returning,
    /// and a brief explanation of how it handles those events.
    /// - [`WlDisplayEvent::DeleteId`]: The corresponding object is removed from the [`WaylandClient`]'s
    /// object storage.
    /// - [`WlRegistryEvent::GlobalRemove`]: The global is removed from both the [`WaylandClient`]'s global
    /// singleton storage and object storage.
    /// - [`WlCallbackEvent::Done`]: The corresponding callback will be called, if there's a function for it.
    pub fn next_event(&mut self, nonblocking: bool) -> Option<WaylandEvent> {
        let mut header = [0_u8; 8];
        self.socket.set_nonblocking(nonblocking).unwrap();
        self.socket.borrow().read_exact(&mut header).ok()?;

        let decoder = WireDecoder::new(&header);
        let msg_len = decoder.len();
        println!("Got message with len {msg_len}");

        self.msg_buffer.extend(header);
        self.msg_buffer.resize(msg_len as usize, 0);
        self.socket
            .borrow()
            .read_exact(&mut self.msg_buffer[8..])
            .expect("Failed to read from Wayland compositor.");

        let mut decoder = WireDecoder::new(&self.msg_buffer);
        let obj_id = decoder.object_id();
        let interface = self.objects[obj_id.raw as usize]
            .expect("Wayland compositor sent an invalid object ID in a message.");
        let event = interface.decode_event(&mut decoder)?;

        match &event {
            WaylandEvent::WlDisplayEvent(WlDisplayEvent::DeleteId(_, id)) => {
                let idx = id.raw as usize;
                if self.objects[idx].take() == Some(Interface::WlCallback) {
                    let _ = self.callbacks.remove(id);
                }
            }
            WaylandEvent::WlRegistryEvent(WlRegistryEvent::GlobalRemove(_, name)) => {
                if let Some(id) = self.names.remove(name) {
                    let idx = id.raw as usize;
                    self.globals.remove(&self.objects[idx].take().unwrap());
                }
            }
            WaylandEvent::WlCallbackEvent(WlCallbackEvent::Done(callback, arg)) => {
                if let Some(cb) = self.callbacks.get(&callback.id) {
                    cb(*arg)
                }
            }
            _ => {}
        }

        self.msg_buffer.clear();
        Some(event)
    }

    /// Send a Wayland message for an object method.
    pub fn call_method<O>(&self, obj: &O, method: O::Methods)
    where
        O: Object,
        O::Methods: ObjectMethods,
    {
        println!("Calling method {} for object {}", method.opcode(), obj.id());
        let mut encoder = WireEncoder::new(obj.id(), method.opcode());
        method.write_wire(&mut encoder);
        let (bytes, fd) = encoder.finish();
        socket::send(&self.socket, &bytes, fd);
    }

    /// Get a global singleton by its type. This will panic if that global singleton wasn't
    /// registered with [`WlRegistry::bind`] - for the non-panicking version, see
    /// [`WaylandClient::try_get_global`].
    #[inline(always)]
    pub fn get_global<G: GlobalSingleton>(&self) -> G {
        self.try_get_global()
            .unwrap_or_else(|| panic!("Global `{}` wasn't registered.", G::INTERFACE))
    }
    /// A non-panicking version of [`WaylandClient::get_global`].
    pub fn try_get_global<G: GlobalSingleton>(&self) -> Option<G> {
        let id = self.globals.get(&G::INTERFACE)?;
        Some(G::new_with_id(*id))
    }

    pub fn bind_global<G: GlobalSingleton>(&mut self, name: Name) -> G {
        let id = self.next_object_id();
        self.call_method(
            &WlRegistry::global(),
            WlRegistryMethod::Bind(
                name,
                NewId {
                    interface: G::INTERFACE,
                    version: G::INTERFACE.version_number(),
                    id,
                },
            ),
        );

        let idx = id.raw as usize;
        self.objects[idx] = Some(G::INTERFACE);
        self.names.insert(name, id);
        self.globals.insert(G::INTERFACE, id);

        G::new_with_id(id)
    }

    /// Get the interface of an object.
    #[inline(always)]
    pub fn object_interface(&self, obj: Id) -> Option<Interface> {
        self.objects.get(obj.raw as usize)?.as_ref().cloned()
    }

    /// The next available object ID.
    pub fn next_object_id(&mut self) -> Id {
        for (idx, obj) in self.objects[1..].iter().enumerate() {
            if obj.is_none() {
                return Id {
                    raw: idx as u32 + 1,
                };
            }
        }

        let raw = self.objects.len() as u32;
        self.objects.push(None);
        Id { raw }
    }
}

/// A trait implemented for all Wayland interfaces.
pub trait Object {
    /// The object's interface.
    const INTERFACE: Interface;
    /// The object's events enum.
    type Events;
    /// The object's methods enum.
    type Methods;
    /// The object's ID.
    fn id(&self) -> Id;
    fn new_with_id(id: Id) -> Self;
}
/// A trait implemented for all interfaces that are global singletons.
pub trait GlobalSingleton: Object {
    const UNUSED: bool; // used for macros
}

/// Adds custom flags to an `OpenOptions` to make it create a tempfile. Specifically, this
/// adds the `O_TMPFILE` flag, which is supported on Linux 3.11 or later.
pub fn create_tmpfile(openoptions: &mut OpenOptions) -> &mut OpenOptions {
    // meaning of constants: man open(2)
    // definition of constants: less /usr/include/bits/fcntl-linux.h
    const O_DIRECTORY: i32 = 0o200000;
    const O_TMPFILE: i32 = 0o20000000 | O_DIRECTORY;

    openoptions.custom_flags(O_TMPFILE).create(false)
}
