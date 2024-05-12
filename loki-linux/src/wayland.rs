#![allow(unused)]

use {
    self::{
        interfaces::{callback::Callback, display::WaylandDisplay, registry::WaylandRegistry},
        wire::MessageHeader,
    },
    std::{
        any::Any,
        cell::{Cell, Ref, RefCell},
        collections::HashMap,
        env,
        io::{Read, Write},
        os::unix::{io::FromRawFd, net::UnixStream},
        path::PathBuf,
        rc::Rc,
    },
    wire::WireType,
};

pub mod interfaces;
pub mod wire;

thread_local! {
    pub static GLOBAL_STORAGE: RefCell<Option<WaylandStorage>> = const { RefCell::new(None) };
}

/// Stores objects, listeners, and the connection to the Wayland compositor for this Wayland session.
pub struct WaylandStorage {
    /// The client's connection to the Wayland compositor.
    pub compositor: UnixStream,
    /// All of the objects that have been created in this Wayland session. Indexes by object ID.
    pub objects: HashMap<u32, Option<Box<dyn Object>>>,
    /// All of the listeners that have been created in this Wayland session. Indexes by object ID.
    pub listeners: HashMap<u32, Option<Rc<dyn Listener>>>,
    /// Sometimes Wayland will refer to an object by a name instead of an ID. This maps names to IDs so we can use either.
    pub name_to_id_map: HashMap<u32, u32>,
    /// The next available object ID.
    pub next_object_id: u32,
}
impl WaylandStorage {
    /// Run a closure that takes the global [`WaylandStorage`] instance.
    pub fn with_global<F: FnOnce(&mut WaylandStorage) -> R, R>(func: F) -> R {
        GLOBAL_STORAGE.with_borrow_mut(|global| func(global.as_mut().unwrap()))
    }

    /// Sends a message in the wire format to the Wayland compositor.
    pub fn send_message(&mut self, sender: &impl Object, opcode: u16, args: &[&dyn WireType]) {
        let message_size = args.iter().map(|val| val.size()).sum::<u16>() + 8;

        let header = MessageHeader {
            object: sender.id(),
            opcode,
            message_size,
        };

        let mut message = header.to_bytes().to_vec();
        args.iter().for_each(|arg| arg.to_wire(&mut message));
        self.compositor.write_all(&message);
    }

    /// Reads the next wire message from the Wayland compositor, if one is available.
    pub fn next_message(&mut self) -> Option<(MessageHeader, Vec<u32>)> {
        let mut header_bytes = [0; 8];
        if self.compositor.read_exact(&mut header_bytes).is_err() {
            return None;
        }
        let header = MessageHeader::from(header_bytes);

        let mut message_bytes = vec![0; header.message_size.div_ceil(4) as usize - 2];
        // Safety: u32s align to u8s perfectly, and we're still respecting ownership rules
        let mut buffer = unsafe { message_bytes.as_mut_slice().align_to_mut().1 };
        self.compositor.read_exact(buffer);

        Some((header, message_bytes))
    }

    /// Allocate an ID for a new object.
    pub fn alloc_object_id(&mut self) -> u32 {
        let id = self.next_object_id;
        self.next_object_id += 1;
        id
    }
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

/// The client that communicates with a Wayland compositor.
pub struct WaylandClient {
    _priv: (),
}
impl WaylandClient {
    /// Attempts to establish a connection to the Wayland compositor.
    pub fn new() -> Option<Self> {
        let compositor = find_compositor()?;
        GLOBAL_STORAGE.with_borrow_mut(|global| {
            *global = Some(WaylandStorage {
                compositor,
                objects: HashMap::default(),
                listeners: HashMap::default(),
                name_to_id_map: HashMap::default(),
                next_object_id: 0,
            });
        });

        // Init global singletons
        // Object 0 is always None. Object 1 is always the display. Loki allocates object 2 to be the registry for convenience.
        WaylandStorage::with_global(|global| {
            global.alloc_object_id();
            global.alloc_object_id();
            global.alloc_object_id();
        });
        WaylandDisplay._msg_get_registry(2);

        Some(Self { _priv: () })
    }

    /// Reads the next message from the Wayland compositor.
    pub fn read_message(&self) {
        let listener = WaylandStorage::with_global(|global| {
            let (header, data) = global.next_message()?;

            if let Some(listener) = global.listeners.get(&header.object) {
                return listener
                    .as_ref()
                    .map(|listener| (listener.clone(), header, data));
            }

            None
        });

        if let Some((listener, header, data)) = listener {
            listener.parse(header, data.as_slice());
        }
    }
}

/// Instances of Wayland interfaces.
pub trait Object {
    /// Get the ID of this object.
    fn id(&self) -> u32;
    /// The name of this object's interface, as a string.
    fn interface(&self) -> &'static str;
    /// The version of the object's interface.
    fn version(&self) -> u32;
}

/// An instance of a global Wayland interface.
pub trait GlobalObject: Object + 'static {
    /// Create the global instance with the specified object ID.
    fn new_with_id(id: u32) -> Self;
}

/// A type to handle events for a specific interface.
pub trait Listener {
    /// Handle the event based on the raw Wayland wire message.
    fn parse(&self, header: MessageHeader, data: &[u32]);
}
