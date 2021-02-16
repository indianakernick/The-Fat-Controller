use super::{os, Error};

impl crate::InfoContext for super::Context {
    fn cursor_location(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let screen = os::XScreenOfDisplay(self.display, self.screen_number);
            let window = os::XRootWindowOfScreen(screen);
            // Passing null pointers for the things we don't need results in a
            // segfault.
            let mut root_return = os::None;
            let mut child_return = os::None;
            let mut root_x_return = 0;
            let mut root_y_return = 0;
            let mut win_x_return = 0;
            let mut win_y_return = 0;
            let mut mask_return = 0;
            if os::XQueryPointer(
                self.display,
                window,
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            ) == os::False {
                Err(Error::XQueryPointer)
            } else {
                Ok((win_x_return as i32, win_y_return as i32))
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
