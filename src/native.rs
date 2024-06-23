#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "ios")]
pub mod ios;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
mod types {
    use super::linux;

    pub type DefaultLokinitBackend = linux::LinuxBackend;
    pub type WindowId = usize;

    #[cfg(feature = "opengl")]
    pub type WindowSurface = linux::opengl::GlSurface;
}
#[cfg(target_os = "macos")]
mod types {
    use super::macos;

    pub type DefaultLokinitBackend = macos::MacosBackend;
    pub type WindowId = isize;

    #[cfg(feature = "opengl")]
    pub type WindowSurface = macos::opengl::WindowSurface;
}

#[cfg(target_os = "windows")]
pub type DefaultLokinitBackend = windows::WindowsBackend;

pub use types::*;
