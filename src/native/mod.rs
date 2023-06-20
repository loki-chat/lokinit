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

// export common stuff

#[cfg(target_os = "android")]
pub use android;

#[cfg(target_os = "ios")]
pub use ios;

#[cfg(target_os = "linux")]
pub type DefaultLokinitBackend = linux::LinuxBackend;

#[cfg(target_os = "macos")]
pub use macos::{CreateWindowError, LokinitCore, NativeCoreError};

#[cfg(target_os = "windows")]
pub use windows;
