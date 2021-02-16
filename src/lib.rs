mod byte_command;
mod command;
mod command_code;
mod info_context;
mod key;
mod key_context;
mod mouse_button;
mod mouse_context;

pub use byte_command::*;
pub use command::*;
pub use command_code::*;
pub use info_context::*;
pub use key::*;
pub use key_context::*;
pub use mouse_button::*;
pub use mouse_context::*;

#[cfg(target_os = "linux")]
mod linux_common;

// How do we detect X11 vs Wayland? Maybe we need a build.rs that sets a config
// attribute?
// There are some solutions here:
// https://unix.stackexchange.com/questions/202891/how-to-know-whether-wayland-or-x11-is-being-used#:~:text=You%20could%20run%20the%20xdpyinfo,in%20a%20pure%20Wayland%20setting).&text=The%20%2DB%20flag%20stands%20for,to%20print%20the%20binary%20name.

#[cfg(wayland)]
mod linux_wayland;
#[cfg(wayland)]
pub use linux_wayland::{Context, Error};

#[cfg(target_os = "linux")]
mod linux_x11;
#[cfg(target_os = "linux")]
pub use linux_x11::{Context, Error};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{Context, Error};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{Context, Error};

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub struct Context;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub struct Error;
