use super::Command;
use crate::{CommandCode, Enum};

trait ToBytes {
    fn byte_size(&self) -> usize;
    fn write_bytes(self, buf: &mut [u8]);
}

impl<T: Enum> ToBytes for T {
    fn byte_size(&self) -> usize {
        1
    }

    fn write_bytes(self, buf: &mut [u8]) {
        buf[0] = self.into_u8()
    }
}

impl ToBytes for u8 {
    fn byte_size(&self) -> usize {
        1
    }

    fn write_bytes(self, buf: &mut [u8]) {
        buf[0] = self;
    }
}

impl ToBytes for i32 {
    fn byte_size(&self) -> usize {
        2
    }

    fn write_bytes(self, buf: &mut [u8]) {
        assert!(i16::MIN as i32 <= self && self <= i16::MAX as i32);
        let val = self as i16;
        buf[0] = (val >> 8) as u8;
        buf[1] = val as u8;
    }
}

impl ToBytes for u32 {
    fn byte_size(&self) -> usize {
        2
    }

    fn write_bytes(self, buf: &mut [u8]) {
        assert!(self <= u16::MAX as u32);
        let val = self as u16;
        buf[0] = (val >> 8) as u8;
        buf[1] = val as u8;
    }
}

impl ToBytes for char {
    fn byte_size(&self) -> usize {
        4
    }

    fn write_bytes(self, buf: &mut [u8]) {
        let val = self as u32;
        buf[0] = (val >> 24) as u8;
        buf[1] = (val >> 16) as u8;
        buf[2] = (val >> 8) as u8;
        buf[3] = val as u8;
    }
}

impl ToBytes for &[u8] {
    fn byte_size(&self) -> usize {
        (self.len() as u32).byte_size() + self.len()
    }

    fn write_bytes(self, buf: &mut [u8]) {
        let self_len = self.len() as u32;
        let size = self_len.byte_size();
        self_len.write_bytes(buf);
        buf[size..size + self.len()].copy_from_slice(self);
    }
}

macro_rules! byte_size_sum {
    ($first:expr) => {
        $first.byte_size()
    };
    ($first:expr, $($rest:expr),+) => {
        $first.byte_size() + byte_size_sum!($($rest),*)
    }
}

macro_rules! write_bytes {
    ($buf:ident, $offset:expr, $first:expr) => {
        $first.write_bytes(&mut $buf[$offset..]);
    };
    ($buf:ident, $offset:expr, $first:expr, $($rest:expr),+) => {
        $first.write_bytes(&mut $buf[$offset..]);
        write_bytes!($buf, $offset + $first.byte_size(), $($rest),*);
    }
}

macro_rules! write_command {
    ($buf:ident, $command_code:tt, $($values:expr),+) => {
        {
            let len = 1 + byte_size_sum!($($values),+);
            if $buf.len() < len {
                return Err(len);
            }
            $buf[0] = CommandCode::$command_code as u8;
            write_bytes!($buf, 1, $($values),+);
            Ok(len)
        }
    }
}

impl Command {
    /// Fill a byte array with the command.
    ///
    /// See [`from_bytes`](Self::from_bytes) for a description of the byte
    /// format. On success, this will return `Ok` with the number of bytes
    /// written. If the given slice is too small, this will return `Err` with
    /// the number of bytes necessary.
    ///
    /// # Examples
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
    /// let total_size = commands.iter().map(|c| c.bytes_len()).sum();
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
            Command::Delay(delay) => write_command!(buf, Delay, *delay),
            Command::KeyDown(key) => write_command!(buf, KeyDown, *key),
            Command::KeyUp(key) => write_command!(buf, KeyUp, *key),
            Command::KeyClick(key) => write_command!(buf, KeyClick, *key),

            Command::MouseMoveRel(x, y) => write_command!(buf, MouseMoveRel, *x, *y),
            Command::MouseMoveAbs(x, y) => write_command!(buf, MouseMoveAbs, *x, *y),
            Command::MouseScroll(x, y) => write_command!(buf, MouseScroll, *x, *y),
            Command::MouseDown(button) => write_command!(buf, MouseDown, *button),
            Command::MouseUp(button) => write_command!(buf, MouseUp, *button),
            Command::MouseClick(button) => write_command!(buf, MouseClick, *button),

            Command::AsciiCharDown(ch) => write_command!(buf, AsciiCharDown, *ch),
            Command::AsciiCharUp(ch) => write_command!(buf, AsciiCharUp, *ch),
            Command::AsciiChar(ch) => write_command!(buf, AsciiChar, *ch),
            Command::AsciiString(string) => write_command!(buf, AsciiString, string.as_slice()),

            Command::UnicodeCharDown(ch) => write_command!(buf, UnicodeCharDown, *ch),
            Command::UnicodeCharUp(ch) => write_command!(buf, UnicodeCharUp, *ch),
            Command::UnicodeChar(ch) => write_command!(buf, UnicodeChar, *ch),
            Command::UnicodeString(string) => write_command!(buf, UnicodeString, string.as_bytes()),
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
