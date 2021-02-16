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

impl Context {
    fn button_event(&mut self, button: c_uint, down: bool) -> Result<(), Error> {
        let press = if down { os::True } else { os::False };
        unsafe {
            os::XTestFakeButtonEvent(self.display, button, press, os::CurrentTime);
            os::XSync(self.display, os::False);
        }
        Ok(())
    }

    fn repeat_button_event(&mut self, count: i32, button: c_uint) {
        unsafe {
            for _ in 0..count {
                os::XTestFakeButtonEvent(self.display, button, os::True, os::CurrentTime);
                os::XTestFakeButtonEvent(self.display, button, os::False, os::CurrentTime);
            }
            os::XSync(self.display, os::False);
        }
    }
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        unsafe {
            os::XTestFakeRelativeMotionEvent(
                self.display,
                self.screen_number,
                dx as c_int,
                dy as c_int,
                os::CurrentTime
            );
        }
        Ok(())
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            os::XTestFakeMotionEvent(
                self.display,
                self.screen_number,
                x as c_int,
                y as c_int,
                os::CurrentTime
            );
        }
        Ok(())
    }

    fn mouse_warp(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.mouse_move_abs(x, y)
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let delta = self.scroll.accumulate(dx, dy);
        if dx < 0 {
            self.repeat_button_event(-delta.0, 6);
        } else if dx > 0 {
            self.repeat_button_event(delta.0, 7);
        }
        if dy < 0 {
            self.repeat_button_event(-delta.1, 4);
        } else if dy > 0 {
            self.repeat_button_event(delta.1, 5);
        }
        Ok(())
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        self.button_event(to_button(button), true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        self.button_event(to_button(button), false)
    }
}
