use std::ptr;
use super::{os, Error};

impl crate::InfoContext for super::Context {
    fn mouse_location(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let screen = os::XScreenOfDisplay(self.display, self.screen_number);
            let window = os::XRootWindowOfScreen(screen);
            let mut x = 0;
            let mut y = 0;
            if os::XQueryPointer(
                self.display,
                window,
                ptr::null_mut(), // root_return
                ptr::null_mut(), // child_return
                ptr::null_mut(), // root_x_return
                ptr::null_mut(), // root_y_return
                &mut x,          // win_x_return
                &mut y,          // win_y_return
                ptr::null_mut(), // mask_return
            ) == os::False {
                Err(Error::QueryPointer)
            } else {
                Ok((x as i32, y as i32))
            }
        }
    }

    fn screen_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let screen = os::XScreenOfDisplay(self.display, self.screen_number);
            let width = os::XWidthOfScreen(screen);
            let height = os::XHeightOfScreen(screen);
            Ok((width as i32, height as i32))
        }
    }
}
