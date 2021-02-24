use super::{ffi, Error, error::PlatformError};

impl crate::InfoContext for super::Context {
    fn cursor_location(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut point = ffi::POINT { x: 0, y: 0 };
            if ffi::GetCursorPos(&mut point) != 0 {
                Ok((point.x as i32, point.y as i32))
            } else {
                Err(Error::Platform(PlatformError::last()))
            }
        }
    }

    fn screen_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let width = ffi::GetSystemMetrics(ffi::SM_CXSCREEN);
            let height = ffi::GetSystemMetrics(ffi::SM_CYSCREEN);
            if width != 0 && height != 0 {
                Ok((width as i32, height as i32))
            } else {
                Err(Error::Unknown)
            }
        }
    }
}
