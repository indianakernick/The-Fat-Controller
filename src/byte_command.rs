use std::fmt;
use crate::{Command, CommandCode, Key, MouseButton};

/// Error enum for [`parse_byte_command`](parse_byte_command).
#[derive(Debug)]
pub enum ParseByteCommandError {
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

use ParseByteCommandError::*;

impl fmt::Display for ParseByteCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidCommandCode(byte) => write!(f, "Invalid command code byte ({})", byte),
            InvalidKey(byte) => write!(f, "Invalid key byte ({})", byte),
            InvalidMouseButton(byte) => write!(f, "Invalid mouse button byte ({})", byte),
            BufferTooShort(len) => write!(f, "Buffer length ({}) is too short", len),
        }
    }
}

impl std::error::Error for ParseByteCommandError {}

fn parse_int(byte_0: u8, byte_1: u8) -> i32 {
    (((byte_0 as i16) << 8) | (byte_1 as i16)) as i32
}

fn parse_command_code(byte: u8) -> Result<CommandCode, ParseByteCommandError> {
    if byte < CommandCode::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidCommandCode(byte))
    }
}

fn parse_key(byte: u8) -> Result<Key, ParseByteCommandError> {
    if byte < Key::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidKey(byte))
    }
}

fn parse_mouse_button(byte: u8) -> Result<MouseButton, ParseByteCommandError> {
    if byte < MouseButton::COUNT {
        unsafe { Ok(std::mem::transmute(byte)) }
    } else {
        Err(InvalidMouseButton(byte))
    }
}

fn check_buffer_length(buf: &[u8], len: usize) -> Result<(), ParseByteCommandError> {
    if buf.len() >= len {
        Ok(())
    } else {
        // Should we put expected, actual, or both?
        Err(BufferTooShort(buf.len()))
    }
}

/// Parse a sequence of bytes to create a [`Command`](Command).
///
/// The first byte in the buffer must be a [`CommandCode`](CommandCode). This
/// identifies the command and its arguments. Following the command identifier
/// is a sequence of bytes that encode the arguments of the command.
/// [`Key`](Key) and [`MouseButton`](MouseButton) are single bytes. For integer
/// arguments (used for moving the mouse and scrolling), 16-bit signed
/// big-endian integers are used.
///
/// Returns the command and the number of bytes that were read from the buffer.
///
/// # Arguments
/// * `buf` - The byte buffer to read from.
///
/// # Examples
///
/// ```
/// let bytes = &[
///     tfc::CommandCode::MouseMoveRel as u8, 255, 214, 0, 64,
///     tfc::CommandCode::KeyClick as u8, tfc::Key::K as u8
/// ];
///
/// let (command, len) = tfc::parse_byte_command(bytes).unwrap();
/// assert_eq!(len, 5);
/// assert_eq!(command, tfc::Command::MouseMoveRel(-42, 64));
///
/// let bytes = &bytes[len..];
/// let (command, len) = tfc::parse_byte_command(bytes).unwrap();
/// assert_eq!(len, 2);
/// assert_eq!(command, tfc::Command::KeyClick(tfc::Key::K));
/// ```
pub fn parse_byte_command(buf: &[u8]) -> Result<(Command, usize), ParseByteCommandError> {
    if buf.len() == 0 {
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
        CommandCode::MouseWarp => {
            check_buffer_length(buf, 5)?;
            Ok((Command::MouseWarp(parse_int(buf[1], buf[2]), parse_int(buf[3], buf[4])), 5))
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
    }
}
