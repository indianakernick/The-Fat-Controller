mod command;
mod command_code;
mod error;
mod key;
mod mouse_button;
mod utils;
pub mod traits;

pub use command::*;
pub use command_code::*;
pub use error::*;
pub use key::*;
pub use mouse_button::*;
pub use traits::*;

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
pub use macos::Context;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{Context, Error};

/// Convenience type alias for [`GenericError`](GenericError).
pub type Error = GenericError<<Context as FallibleContext>::PlatformError>;
