mod execute;
mod from_bytes;
mod to_bytes;

use crate::{Key, MouseButton};

pub use from_bytes::CommandBytesError;

/// A future invocation of a method on a [`Context`](crate::Context).
///
/// Commands can be executed by calling [`execute`](Command::execute).
/// Each variant corresponds to a method on one of the
/// [`traits`](crate::traits).
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    /// Creates a delay for a number of milliseconds.
    Delay(u32),
    /// Corresponds to [`key_down`](crate::KeyboardContext::key_down).
    KeyDown(Key),
    /// Corresponds to [`key_up`](crate::KeyboardContext::key_up).
    KeyUp(Key),
    /// Corresponds to [`key_click`](crate::KeyboardContext::key_click).
    KeyClick(Key),
    /// Corresponds to [`mouse_move_rel`](crate::MouseContext::mouse_move_rel).
    MouseMoveRel(i32, i32),
    /// Corresponds to [`mouse_move_abs`](crate::MouseContext::mouse_move_abs).
    MouseMoveAbs(i32, i32),
    /// Corresponds to [`mouse_scroll`](crate::MouseContext::mouse_scroll).
    MouseScroll(i32, i32),
    /// Corresponds to [`mouse_down`](crate::MouseContext::mouse_down).
    MouseDown(MouseButton),
    /// Corresponds to [`mouse_up`](crate::MouseContext::mouse_up).
    MouseUp(MouseButton),
    /// Corresponds to [`mouse_click`](crate::MouseContext::mouse_click).
    MouseClick(MouseButton),
    /// Corresponds to [`ascii_char_down`](crate::AsciiKeyboardContext::ascii_char_down).
    AsciiCharDown(u8),
    /// Corresponds to [`ascii_char_up`](crate::AsciiKeyboardContext::ascii_char_up).
    AsciiCharUp(u8),
    /// Corresponds to [`ascii_char`](crate::AsciiKeyboardContext::ascii_char).
    AsciiChar(u8),
    /// Corresponds to [`ascii_string`](crate::AsciiKeyboardContext::ascii_string).
    AsciiString(Vec<u8>),
    /// Corresponds to [`unicode_char_down`](crate::UnicodeKeyboardContext::unicode_char_down).
    UnicodeCharDown(char),
    /// Corresponds to [`unicode_char_up`](crate::UnicodeKeyboardContext::unicode_char_up).
    UnicodeCharUp(char),
    /// Corresponds to [`unicode_char`](crate::UnicodeKeyboardContext::unicode_char).
    UnicodeChar(char),
    /// Corresponds to [`unicode_string`](crate::UnicodeKeyboardContext::unicode_string).
    UnicodeString(String),
}
