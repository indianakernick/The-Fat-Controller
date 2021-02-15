use super::{os, Error};

impl crate::InfoContext for super::Context {
    fn mouse_location(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut point = os::POINT { x: 0, y: 0 };
            if os::GetCursorPos(&mut point) != 0 {
                Ok((point.x as i32, point.y as i32))
            } else {
                Err(Error::last())
            }
        }
    }

    fn screen_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let width = os::GetSystemMetrics(os::SM_CXSCREEN);
            let height = os::GetSystemMetrics(os::SM_CYSCREEN);
            if width != 0 && height != 0 {
                Ok((width as i32, height as i32))
            } else {
                Err(Error::unknown())
            }
        }
    }
}
