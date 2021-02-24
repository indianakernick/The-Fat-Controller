use super::{ffi, Error, PlatformError};

impl crate::InfoContext for super::Context {
    fn cursor_location(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let screen = ffi::XScreenOfDisplay(self.display, self.screen_number);
            let window = ffi::XRootWindowOfScreen(screen);
            // Passing null pointers for the things we don't need results in a
            // segfault.
            let mut root_return = ffi::None;
            let mut child_return = ffi::None;
            let mut root_x_return = 0;
            let mut root_y_return = 0;
            let mut win_x_return = 0;
            let mut win_y_return = 0;
            let mut mask_return = 0;
            if ffi::XQueryPointer(
                self.display,
                window,
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            ) == ffi::False {
                Err(Error::Platform(PlatformError::XQueryPointer))
            } else {
                Ok((win_x_return as i32, win_y_return as i32))
            }
        }
    }

    fn screen_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let screen = ffi::XScreenOfDisplay(self.display, self.screen_number);
            let width = ffi::XWidthOfScreen(screen);
            let height = ffi::XHeightOfScreen(screen);
            Ok((width as i32, height as i32))
        }
    }
}
