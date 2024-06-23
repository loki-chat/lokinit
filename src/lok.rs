//! The core of Lokinit, where a Lokinit backend is initialized as a global mutable state.

#![allow(unused)]

use std::sync::atomic::{AtomicBool, Ordering};

use crate::{native::DefaultLokinitBackend, window::ScreenMode};
#[cfg(feature = "opengl")]
use {
    crate::gl::{OpenGlConfig, WindowSurface},
    std::ffi::{c_char, c_void},
};

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

    fn set_screen_mode(&mut self, handle: WindowHandle, screen_mode: ScreenMode);

    fn fetch_monitors(&mut self) -> Vec<Monitor>;

    #[cfg(feature = "opengl")]
    fn create_window_surface(&mut self, window: WindowHandle, cfg: OpenGlConfig) -> WindowSurface;
    #[cfg(feature = "opengl")]
    fn load_opengl_func(&mut self, proc_name: *const c_char) -> *mut c_void;
    #[cfg(feature = "opengl")]
    fn make_surface_active(&self, window: WindowHandle, surface: WindowSurface);
    #[cfg(feature = "opengl")]
    fn flush_surface(&self, window: WindowHandle, surface: WindowSurface);
    #[cfg(feature = "opengl")]
    fn update_surface(&self, surface: WindowSurface);
}

thread_local! {
    static INSTANCE: RefCell<Option<DefaultLokinitBackend>> = const { RefCell::new(None) };
}

/// Initializes Lokinit with a default backend.
pub fn init() {
    INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();

        *instance = Some(DefaultLokinitBackend::init());
    })
}

pub fn with<R>(callback: impl FnOnce(&mut DefaultLokinitBackend) -> R) -> R {
    INSTANCE.with(|instance| {
        let mut instance = instance.borrow_mut();
        let instance = instance.as_mut().expect("Lokinit is not initialized");
        (callback)(instance)
    })
}

pub fn create_window(builder: WindowBuilder) -> Result<WindowHandle, CreateWindowError> {
    with(|instance| instance.create_window(builder))
}

pub fn close_window(handle: WindowHandle) {
    with(|instance| instance.close_window(handle))
}

/// Blocks the thread until it receives a new event from the OS' display server, then processes and
/// returns that event. This will return `None` when there are no more events to be processed.
pub fn poll_event() -> Option<Event> {
    with(|instance| instance.poll_event())
}

pub fn set_screen_mode(handle: WindowHandle, screen_mode: ScreenMode) {
    with(|instance| instance.set_screen_mode(handle, screen_mode))
}

pub fn fetch_monitors() -> Vec<Monitor> {
    with(|instance| instance.fetch_monitors())
}

#[cfg(feature = "opengl")]
fn create_window_surface(window: WindowHandle, cfg: OpenGlConfig) -> WindowSurface {
    with(|instance| instance.create_window_surface(window, cfg))
}
#[cfg(feature = "opengl")]
pub fn load_opengl_func(name: *const c_char) -> *mut c_void {
    with(|instance| instance.load_opengl_func(name))
}
