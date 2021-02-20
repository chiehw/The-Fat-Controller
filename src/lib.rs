mod ascii_key;
mod byte_command;
mod command;
mod command_code;
mod key;
mod mouse_button;
mod utils;
pub mod traits;

pub use ascii_key::*;
pub use byte_command::*;
pub use command::*;
pub use command_code::*;
pub use key::*;
pub use mouse_button::*;
pub use traits::*;

/// Identifies a backend implementation.
pub enum Backend {
    None,
    LinuxWayland,
    LinuxX11,
    MacOS,
    Windows,
}

#[cfg(target_os = "linux")]
mod linux_common;

#[cfg(all(target_os = "linux", not(x11)))]
mod linux_wayland;
#[cfg(all(target_os = "linux", not(x11)))]
pub use linux_wayland::{Context, Error};
#[cfg(all(target_os = "linux", not(x11)))]
pub const BACKEND: Backend = Backend::LinuxWayland;

#[cfg(all(target_os = "linux", x11))]
mod linux_x11;
#[cfg(all(target_os = "linux", x11))]
pub use linux_x11::{Context, Error};
#[cfg(all(target_os = "linux", x11))]
pub const BACKEND: Backend = Backend::LinuxX11;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{Context, Error};
#[cfg(target_os = "macos")]
pub const BACKEND: Backend = Backend::MacOS;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{Context, Error};
#[cfg(target_os = "windows")]
pub const BACKEND: Backend = Backend::Windows;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub struct Context;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub struct Error;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub const BACKEND: Backend = Backend::None;
