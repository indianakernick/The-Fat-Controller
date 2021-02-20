use crate::MouseButton;
use super::{os, Context, Error};
use std::os::raw::{c_uint, c_int};

fn to_button(button: MouseButton) -> c_uint {
    match button {
        MouseButton::Left => 1,
        MouseButton::Right => 3,
        MouseButton::Middle => 2,
    }
}

fn button_event(ctx: &Context, button: c_uint, down: bool) -> Result<(), Error> {
    let press = if down { os::True } else { os::False };
    unsafe {
        if os::XTestFakeButtonEvent(ctx.display, button, press, os::CurrentTime) == 0 {
            return Err(Error::XTestFakeButtonEvent);
        }
        os::XSync(ctx.display, os::False);
    }
    Ok(())
}

fn repeat_button_event(ctx: &Context, count: i32, button: c_uint) -> Result<(), Error> {
    unsafe {
        for _ in 0..count {
            if os::XTestFakeButtonEvent(ctx.display, button, os::True, os::CurrentTime) == 0 {
                return Err(Error::XTestFakeButtonEvent);
            }
            if os::XTestFakeButtonEvent(ctx.display, button, os::False, os::CurrentTime) == 0 {
                return Err(Error::XTestFakeButtonEvent);
            }
        }
        os::XSync(ctx.display, os::False);
    }
    Ok(())
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        unsafe {
            // XTestFakeRelativeMotionEvent seems to only move the mouse
            // vertically. Very odd.
            if os::XWarpPointer(self.display, os::None, os::None, 0, 0, 0, 0, dx as c_int, dy as c_int) == 0 {
                return Err(Error::XWarpPointer);
            }
            os::XFlush(self.display);
        }
        Ok(())
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            // XTestFakeMotionEvent apparently ignores the screen number.
            let window = os::XRootWindow(self.display, self.screen_number);
            if os::XWarpPointer(self.display, os::None, window, 0, 0, 0, 0, x as c_int, y as c_int) == 0 {
                return Err(Error::XWarpPointer);
            }
            os::XFlush(self.display);
        }
        Ok(())
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let delta = self.scroll.accumulate(dx, dy);
        if dx < 0 {
            repeat_button_event(self, -delta.0, 6)?;
        } else if dx > 0 {
            repeat_button_event(self, delta.0, 7)?;
        }
        if dy < 0 {
            repeat_button_event(self, -delta.1, 4)?;
        } else if dy > 0 {
            repeat_button_event(self, delta.1, 5)?;
        }
        Ok(())
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, to_button(button), true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, to_button(button), false)
    }
}
