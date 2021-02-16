use crate::MouseButton;
use super::{os, Context, Error};

impl Context {
    fn mouse_event(&mut self, button: MouseButton, down: bool) -> Result<(), Error> {
        let key = match button {
            MouseButton::Left => os::BTN_LEFT,
            MouseButton::Right => os::BTN_RIGHT,
            MouseButton::Middle => os::BTN_MIDDLE,
        };
        self.write(os::EV_KEY, key, if down { 1 } else { 0 })?;
        self.write_syn_report()
    }
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        self.write(os::EV_REL, os::REL_X, dx)?;
        self.write(os::EV_REL, os::REL_Y, dy)?;
        self.write_syn_report()
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.mouse_move_rel(i32::min_value(), i32::min_value())?;
        self.mouse_move_rel(x, y)
    }

    fn mouse_warp(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.mouse_move_abs(x, y)
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        // self.write(os::EV_REL, os::REL_HWHEEL_HI_RES, dx)?;
        // self.write(os::EV_REL, os::REL_WHEEL_HI_RES, -dy)?;
        let delta = self.scroll.accumulate(dx, dy);
        self.write(os::EV_REL, os::REL_HWHEEL, delta.0)?;
        self.write(os::EV_REL, os::REL_WHEEL, -delta.1)?;
        self.write_syn_report()
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        self.mouse_event(button, true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        self.mouse_event(button, false)
    }
}
