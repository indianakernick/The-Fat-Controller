use super::win32 as win;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Error(win::NonZeroDWORD);

impl Error {
    pub(super) fn new(error_code: u32) -> Self {
        Self(win::NonZeroDWORD::new(error_code).unwrap())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let message_buffer: *const u8 = std::ptr::null();

            let message_length = win::FormatMessage(
                win::FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | win::FORMAT_MESSAGE_FROM_SYSTEM
                    | win::FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null(),
                self.0.get(),
                0,
                std::mem::transmute(&message_buffer),
                0,
                std::ptr::null_mut()
            );

            if message_length == 0 {
                return write!(f, "Error code: {}", self.0.get());
            }

            let message = std::slice::from_raw_parts(message_buffer, message_length as usize);
            let result = match std::str::from_utf8(message) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => write!(f, "Error code: {}", self.0.get()),
            };

            win::LocalFree(std::mem::transmute(message_buffer));

            result
        }
    }
}

impl std::error::Error for Error {}
