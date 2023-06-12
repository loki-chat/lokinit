#![allow(unused)]

use {
    crate::{
        event::Event,
        native::{CreateWindowError, NativeCoreError},
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

pub use crate::native::LokinitCore;
thread_local! {
    // pub static INSTANCE: Rc<RefCell<Option<LokinitCore>>> = Rc::new(RefCell::new(None));
    pub static INSTANCE: RefCell<Option<LokinitCore>> = RefCell::new(None);
}

pub fn with<R>(callback: impl FnOnce(&mut LokinitCore) -> R) -> R {
    INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();

        if instance.is_none() {
            *instance = Some(LokinitCore::init());
        }

        callback(instance.as_mut().unwrap())
    })
}

pub fn create_window(builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
    with(|instance| instance.create_window(builder))
}

pub fn poll_event() -> Option<Event> {
    with(|instance| instance.poll_event())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};

    /// Make sure the native core impl supports all methods, and that they return the correct type
    #[test]
    fn core_has_all_methods() {
        let mut core = LokinitCore::init();
        let window = core.create_window(WindowBuilder::default());
        let monitors = LokinitCore::fetch_monitors();
        let event = core.poll_event();

        assert_eq!(core.type_id(), TypeId::of::<LokinitCore>());
        assert_eq!(window.type_id(), TypeId::of::<WindowHandle>());
        assert_eq!(monitors.type_id(), TypeId::of::<Vec<Monitor>>());
        assert_eq!(
            event.type_id(),
            TypeId::of::<Option<(WindowHandle, Event)>>()
        );
    }
}
