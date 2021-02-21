use crate::Error;

/// A context that supports layout-independent Unicode keyboard events.
///
/// # Platform Differences
///
/// This trait is not implemented for Linux-Wayland.
/// [`AsciiKeyboardContext`](crate::AsciiKeyboardContext) may be used as an
/// alternative. On macOS and Windows,
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) is not equivalent to
/// successive calls to
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string).
/// [`unicode_char`](UnicodeKeyboardContext::unicode_char) is meant to press a
/// key corresponding to a character which means that modifiers can be applied.
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string) is meant to type
/// an arbitrary Unicode string possibily bypassing the keyboard meaning that
/// modifiers cannot be applied. In short, the two functions serve different
/// purposes.
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
///     ctx.unicode_char('a').unwrap()?;
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
///     ctx.unicode_string("a").unwrap()?;
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
/// and are on the current keyboard layout. However,
/// [`unicode_string`](UnicodeKeyboardContext::unicode_string) doesn't have
/// these restrictions and can handle any Unicode string.
pub trait UnicodeKeyboardContext {

    /// Generate a key press and release event along with the necessary
    /// modifiers to type a unicode character.
    ///
    /// Returns `None` if the given character is unsupported.
    ///
    /// # Arguments
    ///
    /// * `ch` - The Unicode character to type.
    fn unicode_char(&mut self, ch: char) -> Option<Result<(), Error>>;

    /// Generate key presses and releases such that a Unicode string is typed.
    ///
    /// If any of the characters in the string are unsupported, `None` will be
    /// returned and no key presses will occur.
    ///
    /// # Arguments
    ///
    /// * `s` - The Unicode string to type.
    fn unicode_string(&mut self, s: &str) -> Option<Result<(), Error>>;
}
