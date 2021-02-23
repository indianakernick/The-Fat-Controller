use std::{thread, time::Duration};
use std::fmt::{self, Display, Formatter};
use crate::{CommandCode, Error, Key, MouseButton, traits::*};

/// A future invocation of a method on a [`Context`](crate::Context).
///
/// Commands can be executed by calling [`execute`](Command::execute).
/// Each variant corresponds to a method on one of the
/// [`traits`](crate::traits).
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
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
    /// Creates a delay for a number of milliseconds
    Delay(u32),
}

/// Error enum for [`Command::from_bytes`](Command::from_bytes).
#[derive(Debug)]
pub enum CommandBytesError {
    /// Encountered a byte that isn't a valid [`CommandCode`](CommandCode).
    InvalidCommandCode(u8),
    /// Encountered a byte that isn't a valid [`Key`](Key).
    InvalidKey(u8),
    /// Encountered a byte that isn't a valid [`MouseButton`](MouseButton).
    InvalidMouseButton(u8),
    /// Expected the buffer to be longer based upon the
    /// [`CommandCode`](CommandCode) byte.
    BufferTooShort(usize),
}

use CommandBytesError::*;

impl Display for CommandBytesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCommandCode(byte) => write!(f, "Invalid command code byte ({})", byte),
            InvalidKey(byte) => write!(f, "Invalid key byte ({})", byte),
            InvalidMouseButton(byte) => write!(f, "Invalid mouse button byte ({})", byte),
            BufferTooShort(len) => write!(f, "Buffer length ({}) is too short", len),
        }
    }
}

impl std::error::Error for CommandBytesError {}

fn parse_int(byte_0: u8, byte_1: u8) -> i32 {
    (((byte_0 as i16) << 8) | (byte_1 as i16)) as i32
}

fn parse_uint(byte_0: u8, byte_1: u8) -> u32 {
    (((byte_0 as u16) << 8) | (byte_1 as u16)) as u32
}

fn parse_command_code(byte: u8) -> Result<CommandCode, CommandBytesError> {
    if byte < CommandCode::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidCommandCode(byte))
    }
}

fn parse_key(byte: u8) -> Result<Key, CommandBytesError> {
    if byte < Key::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidKey(byte))
    }
}

fn parse_mouse_button(byte: u8) -> Result<MouseButton, CommandBytesError> {
    if byte < MouseButton::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidMouseButton(byte))
    }
}

fn check_buffer_length(buf: &[u8], len: usize) -> Result<(), CommandBytesError> {
    if buf.len() >= len {
        Ok(())
    } else {
        // Should we put expected, actual, or both?
        Err(BufferTooShort(buf.len()))
    }
}

impl Command {
    /// Construct a [`Command`](Command) from a sequence of bytes.
    ///
    /// The first byte in the buffer must be a [`CommandCode`](CommandCode).
    /// This identifies the command and its arguments. Following the command
    /// identifier is a sequence of bytes that encode the arguments of the
    /// command. [`Key`](Key) and [`MouseButton`](MouseButton) are single bytes.
    /// For integer arguments (used for moving the mouse and scrolling), 16-bit
    /// signed big-endian integers are used.
    ///
    /// Returns the command and the number of bytes that were read from the
    /// buffer.
    ///
    /// # Arguments
    /// * `buf` - The byte buffer to read from.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfc::{Command, CommandCode, Key};
    ///
    /// let bytes = &[
    ///     CommandCode::MouseMoveRel as u8, 255, 214, 0, 64,
    ///     CommandCode::KeyClick as u8, Key::K as u8
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
    /// ```
    pub fn from_bytes(buf: &[u8]) -> Result<(Command, usize), CommandBytesError> {
        if buf.is_empty() {
            return Err(BufferTooShort(0));
        }

        match parse_command_code(buf[0])? {
            CommandCode::KeyDown => {
                check_buffer_length(buf, 2)?;
                Ok((Command::KeyDown(parse_key(buf[1])?), 2))
            },
            CommandCode::KeyUp => {
                check_buffer_length(buf, 2)?;
                Ok((Command::KeyUp(parse_key(buf[1])?), 2))
            },
            CommandCode::KeyClick => {
                check_buffer_length(buf, 2)?;
                Ok((Command::KeyClick(parse_key(buf[1])?), 2))
            },
            CommandCode::MouseMoveRel => {
                check_buffer_length(buf, 5)?;
                Ok((Command::MouseMoveRel(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
            },
            CommandCode::MouseMoveAbs => {
                check_buffer_length(buf, 5)?;
                Ok((Command::MouseMoveAbs(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
            },
            CommandCode::MouseScroll => {
                check_buffer_length(buf, 5)?;
                Ok((Command::MouseScroll(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
            },
            CommandCode::MouseDown => {
                check_buffer_length(buf, 2)?;
                Ok((Command::MouseDown(parse_mouse_button(buf[1])?), 2))
            },
            CommandCode::MouseUp => {
                check_buffer_length(buf, 2)?;
                Ok((Command::MouseUp(parse_mouse_button(buf[1])?), 2))
            },
            CommandCode::MouseClick => {
                check_buffer_length(buf, 2)?;
                Ok((Command::MouseClick(parse_mouse_button(buf[1])?), 2))
            },
            CommandCode::Delay => {
                check_buffer_length(buf, 3)?;
                Ok((Command::Delay(parse_uint(buf[1], buf[2])), 3))
            },
        }
    }

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// one of the [`traits`](crate::traits).
    pub fn execute<C>(&self, ctx: &mut C) -> Result<(), Error>
        where C: KeyboardContext + MouseContext
    {
        use Command::*;
        match *self {
            KeyDown(key) => ctx.key_down(key),
            KeyUp(key) => ctx.key_up(key),
            KeyClick(key) => ctx.key_click(key),
            MouseMoveRel(dx, dy) => ctx.mouse_move_rel(dx, dy),
            MouseMoveAbs(x, y) => ctx.mouse_move_abs(x, y),
            MouseScroll(dx, dy) => ctx.mouse_scroll(dx, dy),
            MouseDown(button) => ctx.mouse_down(button),
            MouseUp(button) => ctx.mouse_up(button),
            MouseClick(button) => ctx.mouse_click(button),
            Delay(millis) => Ok(thread::sleep(Duration::from_millis(millis as u64))),
        }
    }
}
