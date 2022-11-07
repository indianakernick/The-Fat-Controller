use crate::MouseButton;
use std::{cmp::Ordering, os::raw::{c_uint, c_int}};
use super::{ffi, Context, Error, PlatformError};

fn to_button(button: MouseButton) -> c_uint {
    match button {
        MouseButton::Left => 1,
        MouseButton::Right => 3,
        MouseButton::Middle => 2,
    }
}

fn button_event(ctx: &Context, button: c_uint, down: bool) -> Result<(), Error> {
    let press = if down { ffi::True } else { ffi::False };
    unsafe {
        if ffi::XTestFakeButtonEvent(ctx.display, button, press, ffi::CurrentTime) == 0 {
            return Err(Error::Platform(PlatformError::XTestFakeButtonEvent));
        }
        ffi::XSync(ctx.display, ffi::False);
    }
    Ok(())
}

fn repeat_button_event(ctx: &Context, count: i32, button: c_uint) -> Result<(), Error> {
    unsafe {
        for _ in 0..count {
            if ffi::XTestFakeButtonEvent(ctx.display, button, ffi::True, ffi::CurrentTime) == 0 {
                return Err(Error::Platform(PlatformError::XTestFakeButtonEvent));
            }
            if ffi::XTestFakeButtonEvent(ctx.display, button, ffi::False, ffi::CurrentTime) == 0 {
                return Err(Error::Platform(PlatformError::XTestFakeButtonEvent));
            }
        }
        ffi::XSync(ctx.display, ffi::False);
    }
    Ok(())
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        unsafe {
            // XTestFakeRelativeMotionEvent seems to only move the mouse
            // vertically. Very odd.
            if ffi::XWarpPointer(self.display, ffi::None, ffi::None, 0, 0, 0, 0, dx as c_int, dy as c_int) == 0 {
                return Err(Error::Platform(PlatformError::XWarpPointer));
            }
            ffi::XFlush(self.display);
        }
        Ok(())
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            // XTestFakeMotionEvent apparently ignores the screen number.
            let window = ffi::XRootWindow(self.display, self.screen_number);
            if ffi::XWarpPointer(self.display, ffi::None, window, 0, 0, 0, 0, x as c_int, y as c_int) == 0 {
                return Err(Error::Platform(PlatformError::XWarpPointer));
            }
            ffi::XFlush(self.display);
        }
        Ok(())
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let delta = self.scroll.accumulate(dx, dy);
        match dx.cmp(&0) {
            Ordering::Less => repeat_button_event(self, -delta.0, 6)?,
            Ordering::Greater => repeat_button_event(self, delta.0, 7)?,
            _ => ()
        };
        match dy.cmp(&0) {
            Ordering::Less => repeat_button_event(self, -delta.1, 4)?,
            Ordering::Greater => repeat_button_event(self, delta.1, 5)?,
            _ => ()
        };
        Ok(())
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, to_button(button), true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, to_button(button), false)
    }
}
