use super::os;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Error(os::NonZeroDWORD);

impl Error {
    pub(super) fn last() -> Self {
        unsafe {
            Self(os::NonZeroDWORD::new(os::GetLastError()).unwrap())
        }
    }

    pub(super) fn unknown() -> Self {
        Self(os::NonZeroDWORD::new(28).unwrap()) // The printer is out of paper
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unsafe {
            let message_buffer: os::LPCWSTR = std::ptr::null();

            let message_length = os::FormatMessageW(
                os::FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | os::FORMAT_MESSAGE_FROM_SYSTEM
                    | os::FORMAT_MESSAGE_IGNORE_INSERTS,
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

            // Removing CRLF and period.
            let message_length = (message_length - 3) as usize;
            let message = std::slice::from_raw_parts(message_buffer, message_length);
            let result = write!(f, "{}", String::from_utf16_lossy(message));

            os::LocalFree(std::mem::transmute(message_buffer));

            result
        }
    }
}

impl std::error::Error for Error {}
