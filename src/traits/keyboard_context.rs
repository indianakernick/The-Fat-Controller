use crate::{Key, Error};

/// A context that supports keyboard events.
///
/// # Platform Differences
///
/// `Key::Fn` and `Key::NumpadClear` are supported on macOS only. In the future,
/// they may be named to reflect this or removed entirely.
pub trait KeyboardContext {

    /// Press down a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to press down.
    fn key_down(&mut self, key: Key) -> Result<(), Error>;

    /// Release a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to release.
    fn key_up(&mut self, key: Key) -> Result<(), Error>;

    /// Press and release a key.
    ///
    /// This is equivalent to calling [`key_down`](KeyboardContext::key_down)
    /// followed by [`key_up`](KeyboardContext::key_up).
    ///
    /// # Arguments
    ///
    /// * `key` - The key to press and release.
    fn key_click(&mut self, key: Key) -> Result<(), Error> {
        self.key_down(key)?;
        self.key_up(key)
    }
}
