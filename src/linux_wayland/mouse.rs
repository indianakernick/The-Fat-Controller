use crate::MouseButton;
use super::{ffi, Context, Error};

fn button_event(ctx: &Context, button: MouseButton, down: bool) -> Result<(), Error> {
    let key = match button {
        MouseButton::Left => ffi::BTN_LEFT,
        MouseButton::Right => ffi::BTN_RIGHT,
        MouseButton::Middle => ffi::BTN_MIDDLE,
    };
    ctx.write(ffi::EV_KEY, key, if down { 1 } else { 0 })?;
    ctx.write_syn_report()
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        self.write(ffi::EV_REL, ffi::REL_X, dx)?;
        self.write(ffi::EV_REL, ffi::REL_Y, dy)?;
        self.write_syn_report()
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        self.mouse_move_rel(i32::min_value(), i32::min_value())?;
        self.mouse_move_rel(x, y)
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        // self.write(ffi::EV_REL, ffi::REL_HWHEEL_HI_RES, dx)?;
        // self.write(ffi::EV_REL, ffi::REL_WHEEL_HI_RES, -dy)?;
        let delta = self.scroll.accumulate(dx, dy);
        self.write(ffi::EV_REL, ffi::REL_HWHEEL, delta.0)?;
        self.write(ffi::EV_REL, ffi::REL_WHEEL, -delta.1)?;
        self.write_syn_report()
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, button, true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, button, false)
    }
}
