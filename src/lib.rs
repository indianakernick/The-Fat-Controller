//! The core of TFC is the [`Context`]. To start generating fake input events,
//! you'll need to create a context. The context by itself is basically useless
//! unless you import the [`traits`]. The trait methods return an [`Error`] if
//! something goes wrong. Bringing these three things together, we end up with
//! this.
//!
//! ```no_run
//! use std::{thread, time::Duration};
//! use tfc::{Context, Error, traits::*};
//!
//! fn main() -> Result<(), Error> {
//!     let mut ctx = Context::new()?;
//!     // For OS-specific reasons, it's necessary to wait a moment after
//!     // creating the context before generating events.
//!     thread::sleep(Duration::from_millis(10));
//!     ctx.unicode_string("Hello world!")
//! }
//! ```
//!
//! In addition to the context and its traits, there is also [`Command`]. This
//! represents an action to perform on the context. It's possible to serialize a
//! command, send it over a network, deserialize it and then execute it. In
//! fact, this is what [TFC-server](https://crates.io/crates/tfc-server) does.

mod command;
#[macro_use]
mod r#enum;
mod command_code;
mod generic_error;
mod key;
mod mouse_button;
mod utils;

pub use command::*;
pub use r#enum::*;
pub use command_code::*;
pub use generic_error::*;
pub use key::*;
pub use mouse_button::*;

/// A collection of traits that [`Context`] implements.
pub mod traits;
pub use traits::*;

#[cfg(target_os = "linux")]
mod linux_common;

#[cfg(all(target_os = "linux", not(x11)))]
mod linux_wayland;
#[cfg(all(target_os = "linux", not(x11)))]
pub use linux_wayland::Context;

#[cfg(all(target_os = "linux", x11))]
mod linux_x11;
#[cfg(all(target_os = "linux", x11))]
pub use linux_x11::Context;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::Context;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Context;

/// Convenience type alias for [`GenericError`].
pub type Error = GenericError<<Context as FallibleContext>::PlatformError>;
