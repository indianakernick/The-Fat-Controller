use super::Command;
use crate::CommandCode;

fn write_int(buf: &mut [u8], val: i32) {
    let val = val as i16;
    buf[0] = (val >> 8) as u8;
    buf[1] = val as u8;
}

fn write_uint(buf: &mut [u8], val: u32) {
    let val = val as u16;
    buf[0] = (val >> 8) as u8;
    buf[1] = val as u8;
}

fn write_char(buf: &mut [u8], val: char) {
    let val = val as u32;
    buf[0] = (val >> 24) as u8;
    buf[1] = (val >> 16) as u8;
    buf[2] = (val >> 8) as u8;
    buf[3] = val as u8;
}

fn write_string(buf: &mut [u8], val: &[u8]) {
    write_uint(buf, val.len() as u32);
    buf[2..2 + val.len()].copy_from_slice(val);
}

fn check_buffer_length(buf: &mut [u8], len: usize) -> Result<(), usize> {
    if buf.len() >= len {
        Ok(())
    } else {
        Err(len)
    }
}

impl Command {
    /// Fill a byte array with the command.
    ///
    /// On success, this will return `Ok` with the number of bytes written. If
    /// the given slice is too small, this will return `Err` with the number of
    /// bytes necessary.
    ///
    /// # Example
    ///
    /// ```
    /// use tfc::{Command, CommandCode, Key};
    ///
    /// // Write some commands to a byte array
    ///
    /// let commands = [
    ///     Command::MouseMoveRel(-42, 64),
    ///     Command::KeyClick(Key::K),
    ///     Command::UnicodeString("🤪".to_owned()),
    /// ];
    ///
    /// let total_size = commands.iter().fold(0, |s, c| s + c.bytes_len());
    /// let mut byte_vec = vec![0; total_size];
    /// let mut byte_slice = byte_vec.as_mut_slice();
    ///
    /// for command in commands.iter() {
    ///     let size = command.to_bytes(byte_slice).unwrap();
    ///     byte_slice = &mut byte_slice[size..];
    /// }
    ///
    /// assert_eq!(byte_slice.len(), 0);
    ///
    /// // Read the commands back from the byte array and make sure they match.
    ///
    /// let mut byte_slice = byte_vec.as_slice();
    /// let mut command_index = 0;
    ///
    /// while !byte_slice.is_empty() {
    ///     let (command, size) = Command::from_bytes(byte_slice).unwrap();
    ///     assert_eq!(command, commands[command_index]);
    ///     byte_slice = &byte_slice[size..];
    ///     command_index += 1;
    /// }
    /// ```
    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<usize, usize> {
        // I'm not sure what to do about range errors for cramming a 32-bit
        // value into 16 bits. The assertions are temporary. I think I might
        // change the byte format to use 32-bit integers instead. If I do that,
        // the asserts on the string lengths will probably stay because why are
        // you passing this thing a 4 GB string!

        // While I'm changing things up, it probably makes sense to use
        // little-endian instead of big-endian because that's probably a tiny
        // bit faster.

        match self {
            Command::Delay(delay) => {
                assert!(*delay <= u16::MAX as u32);
                let len = 3;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::Delay as u8;
                write_uint(&mut buf[1..], *delay);
                Ok(len)
            }

            Command::KeyDown(key) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::KeyDown as u8;
                buf[1] = *key as u8;
                Ok(len)
            }
            Command::KeyUp(key) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::KeyUp as u8;
                buf[1] = *key as u8;
                Ok(len)
            }
            Command::KeyClick(key) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::KeyClick as u8;
                buf[1] = *key as u8;
                Ok(len)
            }

            Command::MouseMoveRel(x, y) => {
                assert!(i16::MIN as i32 <= *x && *x <= i32::MAX);
                assert!(i16::MIN as i32 <= *y && *y <= i32::MAX);
                let len = 5;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::MouseMoveRel as u8;
                write_int(&mut buf[1..], *x);
                write_int(&mut buf[3..], *y);
                Ok(len)
            }
            Command::MouseMoveAbs(x, y) => {
                assert!(i16::MIN as i32 <= *x && *x <= i32::MAX);
                assert!(i16::MIN as i32 <= *y && *y <= i32::MAX);
                let len = 5;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::MouseMoveAbs as u8;
                write_int(&mut buf[1..], *x);
                write_int(&mut buf[3..], *y);
                Ok(len)
            }
            Command::MouseScroll(x, y) => {
                assert!(i16::MIN as i32 <= *x && *x <= i32::MAX);
                assert!(i16::MIN as i32 <= *y && *y <= i32::MAX);
                let len = 5;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::MouseScroll as u8;
                write_int(&mut buf[1..], *x);
                write_int(&mut buf[3..], *y);
                Ok(len)
            }
            Command::MouseDown(button) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::MouseDown as u8;
                buf[1] = *button as u8;
                Ok(len)
            }
            Command::MouseUp(button) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::MouseDown as u8;
                buf[1] = *button as u8;
                Ok(len)
            }
            Command::MouseClick(button) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::MouseDown as u8;
                buf[1] = *button as u8;
                Ok(len)
            }

            Command::AsciiCharDown(ch) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::AsciiCharDown as u8;
                buf[1] = *ch;
                Ok(len)
            }
            Command::AsciiCharUp(ch) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::AsciiCharUp as u8;
                buf[1] = *ch;
                Ok(len)
            }
            Command::AsciiChar(ch) => {
                let len = 2;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::AsciiChar as u8;
                buf[1] = *ch;
                Ok(len)
            }
            Command::AsciiString(string) => {
                assert!(string.len() <= u16::MAX as usize);
                let len = 3 + string.len();
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::AsciiString as u8;
                write_string(&mut buf[1..], string.as_slice());
                Ok(len)
            }

            Command::UnicodeCharDown(ch) => {
                let len = 5;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::UnicodeCharDown as u8;
                write_char(&mut buf[1..], *ch);
                Ok(len)
            }
            Command::UnicodeCharUp(ch) => {
                let len = 5;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::UnicodeCharUp as u8;
                write_char(&mut buf[1..], *ch);
                Ok(len)
            }
            Command::UnicodeChar(ch) => {
                let len = 5;
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::UnicodeChar as u8;
                write_char(&mut buf[1..], *ch);
                Ok(len)
            }
            Command::UnicodeString(string) => {
                assert!(string.len() <= u16::MAX as usize);
                let len = 3 + string.len();
                check_buffer_length(buf, len)?;
                buf[0] = CommandCode::UnicodeString as u8;
                write_string(&mut buf[1..], string.as_bytes());
                Ok(len)
            }
        }
    }

    /// Convenience function to get the number of bytes required for
    /// [`to_bytes`](`Self::to_bytes`).
    ///
    /// This simply calls [`to_bytes`](`Self::to_bytes`) with an empty slice.
    pub fn bytes_len(&self) -> usize {
        self.to_bytes(&mut []).unwrap_err()
    }
}
