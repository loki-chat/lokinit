#![allow(unused)]

use {
    crate::{
        event::Event,
        native::{CreateWindowError, LokinitCore, NativeCoreError},
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

thread_local! {
    pub static INSTANCE: RefCell<Option<LokinitCore>> = RefCell::new(None);
}

pub fn with<R>(callback: impl FnOnce(&mut LokinitCore) -> R) -> R {
    INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();
        let instance = instance.get_or_insert_with(|| LokinitCore::init().unwrap());
        (callback)(instance)
    })
}

pub fn create_window(builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
    with(|instance| instance.create_window(builder))
}

pub fn poll_event() -> Option<Event> {
    with(|instance| instance.poll_event())
}

// TODO: implement monitor fetching

// pub fn fetch_monitors() -> Vec<Monitor> {
//     with(|instance| instance.fetch_monitors())
// }
