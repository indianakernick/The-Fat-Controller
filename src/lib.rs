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

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{Context, Error};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{Context, Error};

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub struct Context;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub struct Error;
