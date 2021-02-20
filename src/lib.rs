mod byte_command;
mod command;
mod command_code;
mod info_context;
mod key;
mod keyboard_context;
mod mouse_button;
mod mouse_context;
mod unicode_keyboard_context;
mod utils;

pub use byte_command::*;
pub use command::*;
pub use command_code::*;
pub use info_context::*;
pub use key::*;
pub use keyboard_context::*;
pub use mouse_button::*;
pub use mouse_context::*;
pub use unicode_keyboard_context::*;

#[cfg(target_os = "linux")]
mod linux_common;

#[cfg(all(target_os = "linux", not(x11)))]
mod linux_wayland;
#[cfg(all(target_os = "linux", not(x11)))]
pub use linux_wayland::{Context, Error};

#[cfg(all(target_os = "linux", x11))]
mod linux_x11;
#[cfg(all(target_os = "linux", x11))]
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
