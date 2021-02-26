use crate::{Key, linux_common};
use super::{ffi, Context, Error};

fn key_event(ctx: &Context, key: Key, down: bool) -> Result<(), Error> {
    ctx.write(ffi::EV_KEY, linux_common::to_key_code(key), if down { 1 } else { 0 })?;
    ctx.write_syn_report()
}

impl crate::KeyboardContext for Context {
    fn key_down(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, true)
    }

    fn key_up(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, false)
    }
}

#[cfg(feature = "ascii-fallback")]
use crate::{GenericError, FallibleContext, AsciiKeyboardContext};

#[cfg(feature = "ascii-fallback")]
impl crate::UnicodeKeyboardContext for Context {
    fn unicode_char_down(&mut self, ch: char) -> Result<(), GenericError<Self::PlatformError>> {
        if !ch.is_ascii() {
            return Err(Error::UnsupportedUnicode(ch));
        }
        match self.ascii_char_down(ch as u8) {
            Err(Error::UnsupportedAscii(c)) => Err(Error::UnsupportedUnicode(c as char)),
            other => other,
        }
    }

    fn unicode_char_up(&mut self, ch: char) -> Result<(), GenericError<Self::PlatformError>> {
        if !ch.is_ascii() {
            return Err(Error::UnsupportedUnicode(ch));
        }
        match self.ascii_char_up(ch as u8) {
            Err(Error::UnsupportedAscii(c)) => Err(Error::UnsupportedUnicode(c as char)),
            other => other,
        }
    }

    fn unicode_char(&mut self, ch: char) -> Result<(), GenericError<Self::PlatformError>> {
        if !ch.is_ascii() {
            return Err(Error::UnsupportedUnicode(ch));
        }
        match self.ascii_char(ch as u8) {
            Err(Error::UnsupportedAscii(c)) => Err(Error::UnsupportedUnicode(c as char)),
            other => other,
        }
    }

    fn unicode_string(&mut self, s: &str) -> Result<(), GenericError<Self::PlatformError>> {
        match self.ascii_string(s.as_bytes()) {
            Err(Error::UnsupportedAscii(c)) => Err(Error::UnsupportedUnicode(c as char)),
            other => other,
        }
    }
}
