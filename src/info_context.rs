use crate::Error;

/// A context that supports fetching device information.
///
/// Implemetors of this trait are able to fetch the state of the device.
pub trait InfoContext {

    /// Get the location of the mouse in pixels.
    ///
    /// The first tuple element is the horizontal coordinate with zero being the
    /// left side of the screen. The second tuple element is the vertical
    /// coordinate with zero being the top of the screen.
    fn mouse_location(&self) -> Result<(i32, i32), Error>;

    /// Get the size of the main screen in pixels.
    ///
    /// The first tuple element is the width and the second tuple element is the
    /// height.
    fn screen_size(&self) -> Result<(i32, i32), Error>;
}
