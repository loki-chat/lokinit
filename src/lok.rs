//! The core of Lokinit, where a Lokinit backend is initialized as a global mutable state.

#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};

use crate::native::DefaultLokinitBackend;

use {
    crate::{
        event::Event,
        window::{WindowBuilder, WindowHandle, WindowPos, WindowSize},
    },
    std::{cell::RefCell, rc::Rc},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonitorId(usize);

#[derive(Clone, Debug)]
pub struct Monitor {
    id: MonitorId,
    position: (u32, u32),
    size: (u32, u32),
    hertz: u32,
}

#[derive(Clone, Debug)]
pub struct CreateWindowError(pub Rc<str>);

pub trait LokinitBackend {
    fn init() -> Self
    where
        Self: Sized + 'static;

    fn create_window(&mut self, builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError>;
    fn close_window(&mut self, handle: WindowHandle);

    fn poll_event(&mut self) -> Option<Event>;

    // TODO: implement monitor fetching in native backends
    fn fetch_monitors(&mut self) -> Vec<Monitor> {
        unimplemented!()
    }
}

thread_local! {
    pub static INSTANCE: RefCell<Option<Box<dyn LokinitBackend>>> = RefCell::new(None);
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initializes Lokinit with a default backend.
pub fn init() {
    init_backend::<DefaultLokinitBackend>()
}

pub fn init_backend<B: LokinitBackend + 'static>() {
    INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();
        let instance = instance.get_or_insert_with(|| {
            if INITIALIZED.load(Ordering::Relaxed) {
                panic!("Lokinit core has already been initialized");
            }

            let backend = B::init();
            INITIALIZED.store(true, Ordering::Release);
            Box::new(backend)
        });
    })
}

pub fn with<R>(callback: impl FnOnce(&mut dyn LokinitBackend) -> R) -> R {
    INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();
        let instance = instance.as_deref_mut().expect("Lokinit is not initialized");
        (callback)(instance)
    })
}

pub fn create_window(builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
    with(|instance| instance.create_window(builder))
}

pub fn close_window(handle: WindowHandle) {
    with(|instance| instance.close_window(handle))
}

pub fn poll_event() -> Option<Event> {
    with(|instance| instance.poll_event())
}

pub fn fetch_monitors() -> Vec<Monitor> {
    with(|instance| instance.fetch_monitors())
}
