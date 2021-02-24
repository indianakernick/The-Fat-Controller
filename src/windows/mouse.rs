use super::{ffi, Context, Error};
use crate::{MouseButton, ScreenContext};

fn button_event(ctx: &Context, button: MouseButton, down: bool) -> Result<(), Error> {
    use MouseButton::*;
    let event = match (button, down) {
        (Left, true) => ffi::MOUSEEVENTF_LEFTDOWN,
        (Left, false) => ffi::MOUSEEVENTF_LEFTUP,
        (Right, true) => ffi::MOUSEEVENTF_RIGHTDOWN,
        (Right, false) => ffi::MOUSEEVENTF_RIGHTUP,
        (Middle, true) => ffi::MOUSEEVENTF_MIDDLEDOWN,
        (Middle, false) => ffi::MOUSEEVENTF_MIDDLEUP,
    };
    let mut input = ffi::INPUT::default();
    input.type_ = ffi::INPUT_MOUSE;
    input.u.mi.dwFlags = event;
    ctx.send_input(&input)
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let mut input = ffi::INPUT::default();
        input.type_ = ffi::INPUT_MOUSE;
        input.u.mi.dx = dx as ffi::LONG;
        input.u.mi.dy = dy as ffi::LONG;
        input.u.mi.dwFlags = ffi::MOUSEEVENTF_MOVE;
        self.send_input(&input)
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        let screen = self.screen_size()?;
        let screen = (screen.0 - 1, screen.1 - 1);
        let mut input = ffi::INPUT::default();
        input.type_ = ffi::INPUT_MOUSE;
        input.u.mi.dwFlags = ffi::MOUSEEVENTF_MOVE | ffi::MOUSEEVENTF_ABSOLUTE;
        input.u.mi.dx = (x * 65535 + screen.0 / 2) / screen.0;
        input.u.mi.dy = (y * 65535 + screen.1 / 2) / screen.1;
        self.send_input(&input)
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let mut input = ffi::INPUT::default();
        input.type_ = ffi::INPUT_MOUSE;

        if dx != 0 {
            input.u.mi.dwFlags = ffi::MOUSEEVENTF_HWHEEL;
            input.u.mi.mouseData = dx as ffi::DWORD;
            self.send_input(&input)?;
        }

        if dy != 0 {
            input.u.mi.dwFlags = ffi::MOUSEEVENTF_WHEEL;
            input.u.mi.mouseData = -dy as ffi::DWORD;
            self.send_input(&input)?;
        }

        Ok(())
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, button, true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        button_event(self, button, false)
    }
}
