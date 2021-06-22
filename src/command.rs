use std::{thread, time::Duration};
use std::fmt::{self, Display, Formatter};
use crate::{CommandCode, Key, MouseButton, GenericError, Enum, traits::*};

/// A future invocation of a method on a [`Context`](crate::Context).
///
/// Commands can be executed by calling [`execute`](Command::execute).
/// Each variant corresponds to a method on one of the
/// [`traits`](crate::traits).
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    /// Creates a delay for a number of milliseconds
    Delay(u32),
    /// Corresponds to [`key_down`](KeyboardContext::key_down)
    KeyDown(Key),
    /// Corresponds to [`key_up`](KeyboardContext::key_up)
    KeyUp(Key),
    /// Corresponds to [`key_click`](KeyboardContext::key_click)
    KeyClick(Key),
    /// Corresponds to [`mouse_move_rel`](MouseContext::mouse_move_rel)
    MouseMoveRel(i32, i32),
    /// Corresponds to [`mouse_move_abs`](MouseContext::mouse_move_abs)
    MouseMoveAbs(i32, i32),
    /// Corresponds to [`mouse_scroll`](MouseContext::mouse_scroll)
    MouseScroll(i32, i32),
    /// Corresponds to [`mouse_down`](MouseContext::mouse_down)
    MouseDown(MouseButton),
    /// Corresponds to [`mouse_up`](MouseContext::mouse_up)
    MouseUp(MouseButton),
    /// Corresponds to [`mouse_click`](MouseContext::mouse_click)
    MouseClick(MouseButton),
    /// Corresponds to [`ascii_char_down`](AsciiKeyboardContext::ascii_char_down)
    AsciiCharDown(u8),
    /// Corresponds to [`ascii_char_up`](AsciiKeyboardContext::ascii_char_up)
    AsciiCharUp(u8),
    /// Corresponds to [`ascii_char`](AsciiKeyboardContext::ascii_char)
    AsciiChar(u8),
    /// Corresponds to [`ascii_string`](AsciiKeyboardContext::ascii_string)
    AsciiString(Vec<u8>),
    /// Corresponds to [`unicode_char_down`](UnicodeKeyboardContext::unicode_char_down)
    UnicodeCharDown(char),
    /// Corresponds to [`unicode_char_up`](UnicodeKeyboardContext::unicode_char_up)
    UnicodeCharUp(char),
    /// Corresponds to [`unicode_char`](UnicodeKeyboardContext::unicode_char)
    UnicodeChar(char),
    /// Corresponds to [`unicode_string`](UnicodeKeyboardContext::unicode_string)
    UnicodeString(String),
}

/// Error enum returned by [`Command::from_bytes`].
#[derive(Debug)]
pub enum CommandBytesError {
    /// Encountered a byte that isn't a valid [`CommandCode`].
    InvalidCommandCode(u8),
    /// Encountered a byte that isn't a valid [`Key`].
    InvalidKey(u8),
    /// Encountered a byte that isn't a valid [`MouseButton`].
    InvalidMouseButton(u8),
    /// Encountered a byte sequence that isn't a valid Unicode scalar
    InvalidUnicodeScalar(u32),
    /// Encountered a byte sequence that isn't a valid UTF-8 string
    InvalidUTF8,
    /// Expected the buffer to be longer. Stores the expected length.
    BufferTooShort(usize),
}

use CommandBytesError::*;

impl Display for CommandBytesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCommandCode(byte) => write!(f, "Invalid command code byte ({})", byte),
            InvalidKey(byte) => write!(f, "Invalid key byte ({})", byte),
            InvalidMouseButton(byte) => write!(f, "Invalid mouse button byte ({})", byte),
            InvalidUnicodeScalar(ucs) => write!(f, "Invalid Unicode scalar ({:#010X})", ucs),
            InvalidUTF8 => write!(f, "Invalid UTF-8 string"),
            BufferTooShort(len) => write!(f, "Expected buffer to be at least {} bytes in length", len),
        }
    }
}

impl std::error::Error for CommandBytesError {}

fn parse_int(b_0: u8, b_1: u8) -> i32 {
    (((b_0 as i16) << 8) | (b_1 as i16)) as i32
}

fn parse_uint(b_0: u8, b_1: u8) -> u32 {
    (((b_0 as u16) << 8) | (b_1 as u16)) as u32
}

fn parse_char(b_0: u8, b_1: u8, b_2: u8, b_3: u8) -> Result<char, CommandBytesError> {
    let ch = ((b_0 as u32) << 24) | ((b_1 as u32) << 16) | ((b_2 as u32) << 8) | (b_3 as u32);
    match std::char::from_u32(ch) {
        Some(c) => Ok(c),
        None => Err(InvalidUnicodeScalar(ch)),
    }
}

fn parse_string(buf: &[u8]) -> Result<String, CommandBytesError> {
    match String::from_utf8(buf.to_owned()) {
        Ok(s) => Ok(s),
        Err(_) => Err(InvalidUTF8),
    }
}

fn parse_command_code(byte: u8) -> Result<CommandCode, CommandBytesError> {
    CommandCode::from_u8(byte).ok_or(InvalidCommandCode(byte))
}

fn parse_key(byte: u8) -> Result<Key, CommandBytesError> {
    Key::from_u8(byte).ok_or(InvalidKey(byte))
}

fn parse_mouse_button(byte: u8) -> Result<MouseButton, CommandBytesError> {
    MouseButton::from_u8(byte).ok_or(InvalidMouseButton(byte))
}

fn check_buffer_length(buf: &[u8], len: usize) -> Result<(), CommandBytesError> {
    if buf.len() >= len {
        Ok(())
    } else {
        Err(BufferTooShort(len))
    }
}

impl Command {
    /// Construct a [`Command`] from a sequence of bytes.
    ///
    /// The first byte in the buffer must be a [`CommandCode`]. This identifies
    /// the command and its arguments. Following the command identifier is a
    /// sequence of bytes that encode the arguments of the command. [`Key`] and
    /// [`MouseButton`] are single bytes. For integer arguments (used for moving
    /// the mouse and scrolling), signed 16-bit big-endian integers are used.
    ///
    /// Since a negative delay is impossible, the millisecond parameter for the
    /// `Delay` command is unsigned.
    ///
    /// An ASCII character is a single byte. An ASCII string is a length
    /// followed by a sequence of bytes. The length is an unsigned 16-bit
    /// big-endian integer.
    ///
    /// A Unicode character (or more specifically, a Unicode scalar value) is
    /// 32-bits. A Unicode string is similar to an ASCII string in that it has a
    /// length followed by a sequence of bytes, however the sequence of bytes
    /// are a UTF-8 encoded string.
    ///
    /// The function returns the command and the number of bytes that were read
    /// from the buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use tfc::{Command, CommandCode, Key};
    ///
    /// let bytes = &[
    ///     CommandCode::MouseMoveRel as u8, 255, 214, 0, 64,
    ///     CommandCode::KeyClick as u8, Key::K as u8,
    ///     CommandCode::UnicodeString as u8, 0, 4, 0xF0, 0x9F, 0xA4, 0xAA,
    /// ];
    ///
    /// let (command, len) = Command::from_bytes(bytes).unwrap();
    /// assert_eq!(len, 5);
    /// assert_eq!(command, Command::MouseMoveRel(-42, 64));
    ///
    /// let bytes = &bytes[len..];
    /// let (command, len) = Command::from_bytes(bytes).unwrap();
    /// assert_eq!(len, 2);
    /// assert_eq!(command, Command::KeyClick(Key::K));
    ///
    /// let bytes = &bytes[len..];
    /// let (command, len) = Command::from_bytes(bytes).unwrap();
    /// assert_eq!(len, 7);
    /// assert_eq!(command, Command::UnicodeString("🤪".to_owned()));
    /// ```
    pub fn from_bytes(buf: &[u8]) -> Result<(Command, usize), CommandBytesError> {
        if buf.is_empty() {
            return Err(BufferTooShort(1));
        }

        match parse_command_code(buf[0])? {
            CommandCode::Delay => {
                check_buffer_length(buf, 3)?;
                Ok((Command::Delay(parse_uint(buf[1], buf[2])), 3))
            }

            CommandCode::KeyDown => {
                check_buffer_length(buf, 2)?;
                Ok((Command::KeyDown(parse_key(buf[1])?), 2))
            }
            CommandCode::KeyUp => {
                check_buffer_length(buf, 2)?;
                Ok((Command::KeyUp(parse_key(buf[1])?), 2))
            }
            CommandCode::KeyClick => {
                check_buffer_length(buf, 2)?;
                Ok((Command::KeyClick(parse_key(buf[1])?), 2))
            }

            CommandCode::MouseMoveRel => {
                check_buffer_length(buf, 5)?;
                Ok((Command::MouseMoveRel(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
            }
            CommandCode::MouseMoveAbs => {
                check_buffer_length(buf, 5)?;
                Ok((Command::MouseMoveAbs(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
            }
            CommandCode::MouseScroll => {
                check_buffer_length(buf, 5)?;
                Ok((Command::MouseScroll(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
            }
            CommandCode::MouseDown => {
                check_buffer_length(buf, 2)?;
                Ok((Command::MouseDown(parse_mouse_button(buf[1])?), 2))
            }
            CommandCode::MouseUp => {
                check_buffer_length(buf, 2)?;
                Ok((Command::MouseUp(parse_mouse_button(buf[1])?), 2))
            }
            CommandCode::MouseClick => {
                check_buffer_length(buf, 2)?;
                Ok((Command::MouseClick(parse_mouse_button(buf[1])?), 2))
            }

            CommandCode::AsciiCharDown => {
                check_buffer_length(buf, 2)?;
                Ok((Command::AsciiCharDown(buf[1]), 2))
            }
            CommandCode::AsciiCharUp => {
                check_buffer_length(buf, 2)?;
                Ok((Command::AsciiCharUp(buf[1]), 2))
            }
            CommandCode::AsciiChar => {
                check_buffer_length(buf, 2)?;
                Ok((Command::AsciiChar(buf[1]), 2))
            }
            CommandCode::AsciiString => {
                check_buffer_length(buf, 3)?;
                let len = 3 + parse_uint(buf[1], buf[2]) as usize;
                check_buffer_length(buf, len)?;
                Ok((Command::AsciiString(buf[3..len].to_owned()), len))
            }

            CommandCode::UnicodeCharDown => {
                check_buffer_length(buf, 5)?;
                Ok((Command::UnicodeCharDown(parse_char(buf[1], buf[2], buf[3], buf[4])?), 5))
            }
            CommandCode::UnicodeCharUp => {
                check_buffer_length(buf, 5)?;
                Ok((Command::UnicodeCharUp(parse_char(buf[1], buf[2], buf[3], buf[4])?), 5))
            }
            CommandCode::UnicodeChar => {
                check_buffer_length(buf, 5)?;
                Ok((Command::UnicodeChar(parse_char(buf[1], buf[2], buf[3], buf[4])?), 5))
            }
            CommandCode::UnicodeString => {
                check_buffer_length(buf, 3)?;
                let len = 3 + parse_uint(buf[1], buf[2]) as usize;
                check_buffer_length(buf, len)?;
                Ok((Command::UnicodeString(parse_string(&buf[3..len])?), len))
            }
        }
    }

    fn execute_core<C>(&self, ctx: &mut C) -> Result<bool, GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        use Command::*;
        match self {
            KeyDown(key) => ctx.key_down(*key),
            KeyUp(key) => ctx.key_up(*key),
            KeyClick(key) => ctx.key_click(*key),
            MouseMoveRel(dx, dy) => ctx.mouse_move_rel(*dx, *dy),
            MouseMoveAbs(x, y) => ctx.mouse_move_abs(*x, *y),
            MouseScroll(dx, dy) => ctx.mouse_scroll(*dx, *dy),
            MouseDown(button) => ctx.mouse_down(*button),
            MouseUp(button) => ctx.mouse_up(*button),
            MouseClick(button) => ctx.mouse_click(*button),
            AsciiCharDown(ch) => ctx.ascii_char_down(*ch),
            AsciiCharUp(ch) => ctx.ascii_char_up(*ch),
            AsciiChar(ch) => ctx.ascii_char(*ch),
            AsciiString(s) => ctx.ascii_string(s.as_slice()),
            _ => return Ok(false)
        }?;
        Ok(true)
    }

    #[cfg(not(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    )))]
    fn execute_unicode<C>(&self, ctx: &mut C) -> Result<bool, GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext + UnicodeKeyboardContext
    {
        if self.execute_core(ctx)? {
            return Ok(true);
        }

        use Command::*;
        match self {
            UnicodeCharDown(ch) => ctx.unicode_char_down(*ch),
            UnicodeCharUp(ch) => ctx.unicode_char_up(*ch),
            UnicodeChar(ch) => ctx.unicode_char(*ch),
            UnicodeString(s) => ctx.unicode_string(s.as_str()),
            _ => return Ok(false)
        }?;

        Ok(true)
    }

    #[cfg(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    ))]
    pub fn execute_unicode<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        if self.execute_core(ctx)? {
            return Ok(true);
        }

        use Command::*;
        match self {
            UnicodeCharDown(_) => panic!("UnicodeKeyboardContext is not implemented"),
            UnicodeCharUp(_) => panic!("UnicodeKeyboardContext is not implemented"),
            UnicodeChar(_) => panic!("UnicodeKeyboardContext is not implemented"),
            UnicodeString(_) => panic!("UnicodeKeyboardContext is not implemented"),
            _ => return Ok(false)
        }?;

        Ok(true)
    }

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being synchronous and is
    /// executed using [`std::thread::sleep`](std::thread::sleep).
    #[cfg(not(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    )))]
    pub fn execute<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext + UnicodeKeyboardContext
    {
        if self.execute_unicode(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(thread::sleep(Duration::from_millis(*millis as u64))),
            _ => std::unreachable!()
        }
    }

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being asynchronous and is
    /// executed using [`tokio::time::sleep`](tokio::time::sleep).
    #[cfg(all(
        not(all(
            not(feature = "ascii-fallback"),
            target_os = "linux",
            not(x11)
        )),
        feature = "tokio"
    ))]
    pub async fn execute_async<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext + UnicodeKeyboardContext
    {
        if self.execute_core(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(tokio::time::sleep(Duration::from_millis(*millis as u64)).await),
            _ => std::unreachable!()
        }
    }

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being synchronous and is
    /// executed using [`std::thread::sleep`](std::thread::sleep).
    #[cfg(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11)
    ))]
    pub fn execute<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        if self.execute_unicode(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(thread::sleep(Duration::from_millis(*millis as u64))),
            _ => std::unreachable!()
        }
    }

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    ///
    /// [`Delay`](Command::Delay) is treated as being asynchronous and is
    /// executed using [`tokio::time::sleep`](tokio::time::sleep).
    #[cfg(all(
        not(feature = "ascii-fallback"),
        target_os = "linux",
        not(x11),
        feature = "tokio"
    ))]
    pub async fn execute_async<C>(&self, ctx: &mut C) -> Result<(), GenericError<C::PlatformError>>
        where C: FallibleContext + KeyboardContext + MouseContext + AsciiKeyboardContext
    {
        if self.execute_unicode(ctx)? {
            return Ok(());
        }

        use Command::*;
        match self {
            Delay(millis) => Ok(tokio::time::sleep(Duration::from_millis(*millis as u64)).await),
            _ => std::unreachable!()
        }
    }
}
