use crate::Error;

/// A context that supports fetching device information.
///
/// # Platform Differences
///
/// This trait isn't implemented for the Wayland (or more accurately: not X11)
/// context. It is simply not possible to obtain this information under Wayland.
pub trait InfoContext {

    /// Get the location of the cursor in pixels.
    ///
    /// The first tuple element is the horizontal coordinate with zero being the
    /// left side of the screen. The second tuple element is the vertical
    /// coordinate with zero being the top of the screen.
    fn cursor_location(&self) -> Result<(i32, i32), Error>;

    /// Get the size of the main screen in pixels.
    ///
    /// The first tuple element is the width and the second tuple element is the
    /// height.
    fn screen_size(&self) -> Result<(i32, i32), Error>;
}
