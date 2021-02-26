use super::FallibleContext;
use crate::{Key, GenericError};

/// A context that supports keyboard events.
///
/// # Platform Differences
///
/// `Key::Fn` and `Key::NumpadClear` are supported on macOS only. In the future,
/// they may be named to reflect this or removed entirely.
pub trait KeyboardContext: FallibleContext {

    /// Press down a key.
    fn key_down(&mut self, key: Key) -> Result<(), GenericError<Self::PlatformError>>;

    /// Release a key.
    fn key_up(&mut self, key: Key) -> Result<(), GenericError<Self::PlatformError>>;

    /// Press and release a key.
    ///
    /// This is equivalent to calling [`key_down`](KeyboardContext::key_down)
    /// followed by [`key_up`](KeyboardContext::key_up). Although, some
    /// platforms may optimize this.
    fn key_click(&mut self, key: Key) -> Result<(), GenericError<Self::PlatformError>> {
        self.key_down(key)?;
        self.key_up(key)
    }
}
