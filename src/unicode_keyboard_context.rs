use crate::Error;

/// A context that supports layout-independent unicode keyboard events.
pub trait UnicodeKeyboardContext {
    fn unicode_char(&mut self, ch: char) -> Result<(), Error>;

    fn unicode_string(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.chars() {
            self.unicode_char(ch)?;
        }
        Ok(())
    }
}
