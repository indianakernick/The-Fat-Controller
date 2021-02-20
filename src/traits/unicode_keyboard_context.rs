use crate::Error;

/// A context that supports layout-independent unicode keyboard events.
///
/// # Platform Differences
///
/// This trait is not implemented for Linux-Wayland. On macOS and Windows,
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) is not equivalent to
/// successive calls to
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string).
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) can have modifiers
/// applied to it.
///
/// The following snippet will do a select-all on any keyboard layout. Note that
/// we're using a generic function here because
/// [`UnicodeKeyboardContext`](UnicodeKeyboardContext) is not implemented when
/// compiling for `docs.rs`.
/// ```no_run
/// use tfc::{Error, Key, KeyboardContext, UnicodeKeyboardContext};
///
/// fn select_all<C>(ctx: &mut C) -> Result<(), Error>
///     where C: KeyboardContext + UnicodeKeyboardContext
/// {
///     ctx.key_down(Key::ControlOrMeta)?;
///     ctx.unicode_char('a')?;
///     ctx.key_up(Key::ControlOrMeta)
/// }
/// ```
///
/// However, the next snippet will only do a select-all on Linux-X11.
/// ```no_run
/// use tfc::{Context, Error, Key, traits::*};
///
/// fn select_all<C>(ctx: &mut C) -> Result<(), Error>
///     where C: KeyboardContext + UnicodeKeyboardContext
/// {
///     ctx.key_down(Key::ControlOrMeta)?;
///     ctx.unicode_string("a")?;
///     ctx.key_up(Key::ControlOrMeta)
/// }
/// ```
///
/// Care must be taken when using
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) in this mannor. If an
/// uppercase `'A'` was used, the shift key would have been pressed which may
/// not have had the desired effect.
///
/// On Windows, [`unicode_char`](UnicodeKeyboardContext::unicode_char) is
/// limited to characters that can be represented as a single UTF-16 code unit
/// but [`unicode_string`](UnicodeKeyboardContext::unicode_string) does not have
/// this restriction.
pub trait UnicodeKeyboardContext {

    /// Generate a key press and release event along with the necessary
    /// modifiers to type a unicode character.
    ///
    /// # Arguments
    ///
    /// * `ch` - The unicode character to generate.
    fn unicode_char(&mut self, ch: char) -> Result<(), Error>;

    /// Generate key presses and releases such that a unicode string is typed.
    ///
    /// # Arguments
    ///
    /// * `s` - The unicode string to generate.
    fn unicode_string(&mut self, s: &str) -> Result<(), Error>;
}
