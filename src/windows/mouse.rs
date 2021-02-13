use std::os::raw::c_int;
use super::win32 as win;
use super::{Context, Error};
use crate::{MouseButton, InfoContext};

impl Context {
    fn mouse_button_event(&mut self, button: MouseButton, down: bool) -> Result<(), Error> {
        use MouseButton::*;
        let event = match (button, down) {
            (Left, true) => win::MOUSEEVENTF_LEFTDOWN,
            (Left, false) => win::MOUSEEVENTF_LEFTUP,
            (Right, true) => win::MOUSEEVENTF_RIGHTDOWN,
            (Right, false) => win::MOUSEEVENTF_RIGHTUP,
            (Middle, true) => win::MOUSEEVENTF_MIDDLEDOWN,
            (Middle, false) => win::MOUSEEVENTF_MIDDLEUP,
        };
        let mut input = win::INPUT::default();
        input.type_ = win::INPUT_MOUSE;
        input.u.mi.dwFlags = event;
        self.send_input(&input)
    }
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let mut input = win::INPUT::default();
        input.type_ = win::INPUT_MOUSE;
        input.u.mi.dx = dx as win::LONG;
        input.u.mi.dy = dy as win::LONG;
        input.u.mi.dwFlags = win::MOUSEEVENTF_MOVE;
        self.send_input(&input)
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        let screen = self.screen_size()?;
        let screen = (screen.0 - 1, screen.1 - 1);
        let mut input = win::INPUT::default();
        input.type_ = win::INPUT_MOUSE;
        input.u.mi.dwFlags = win::MOUSEEVENTF_MOVE | win::MOUSEEVENTF_ABSOLUTE;
        input.u.mi.dx = (x * 65535 + screen.0 / 2) / screen.0;
        input.u.mi.dy = (y * 65535 + screen.1 / 2) / screen.1;
        self.send_input(&input)
    }

    fn mouse_warp(&mut self, x: i32, y: i32) -> Result<(), Error> {
        unsafe {
            if win::SetCursorPos(x as c_int, y as c_int) != 0 {
                Ok(())
            } else {
                Err(Error::last())
            }
        }
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let mut input = win::INPUT::default();
        input.type_ = win::INPUT_MOUSE;

        if dx != 0 {
            input.u.mi.dwFlags = win::MOUSEEVENTF_HWHEEL;
            input.u.mi.mouseData = dx as win::DWORD;
            self.send_input(&input)?;
        }

        if dy != 0 {
            input.u.mi.dwFlags = win::MOUSEEVENTF_WHEEL;
            input.u.mi.mouseData = -dy as win::DWORD;
            self.send_input(&input)?;
        }

        Ok(())
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        self.mouse_button_event(button, true)
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        self.mouse_button_event(button, false)
    }
}
