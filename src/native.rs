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

#[cfg(all(target_os = "linux", feature = "opengl"))]
pub type GLSurface = linux::opengl::GlxSurface;

#[cfg(target_os = "macos")]
pub type DefaultLokinitBackend = macos::MacosBackend;

#[cfg(all(target_os = "macos", feature = "opengl"))]
pub type GLSurface = macos::opengl::WindowSurface;

#[cfg(target_os = "windows")]
pub type DefaultLokinitBackend = windows::WindowsBackend;
