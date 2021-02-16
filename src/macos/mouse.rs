use super::{os, Context, Error};
use crate::{MouseButton, InfoContext};

// Largely adapted from here
// https://github.com/ccMSC/ckb/blob/master/src/ckb-daemon/input_mac.c

impl Context {
    fn mouse_event(&mut self, event_type: u32, button_number: u8, down: bool) -> Result<(), Error> {
        let mut event = os::NXEventData::default();
        event.compound.subType = os::NX_SUBTYPE_AUX_MOUSE_BUTTONS;
        unsafe {
            event.compound.misc.L[0] = 1 << button_number;
            event.compound.misc.L[1] = if down { 1 << button_number } else { 0 };
        }

        self.post_event(os::NX_SYSDEFINED, &event, 0, 0)?;

        event = os::NXEventData::default();
        event.mouse.buttonNumber = button_number;

        self.post_event(event_type, &event, 0, 0)
    }
}

impl crate::MouseContext for Context {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let mut event = os::NXEventData::default();
        event.mouseMove.dx = dx;
        event.mouseMove.dy = dy;

        let mut event_type = os::NX_MOUSEMOVED;
        if self.button_state & 0b1 != 0 {
            event_type = os::NX_LMOUSEDRAGGED;
        } else if self.button_state & 0b10 != 0 {
            event_type = os::NX_RMOUSEDRAGGED;
        } else if self.button_state & 0b100 != 0 {
            event_type = os::NX_OMOUSEDRAGGED;
        }

        self.post_event(event_type, &event, 0, os::kIOHIDSetRelativeCursorPosition)
    }

    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error> {
        let location = self.cursor_location()?;
        self.mouse_move_rel(x - location.0, y - location.1)
    }

    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error> {
        let mut event = os::NXEventData::default();
        event.scrollWheel.fixedDeltaAxis1 = dy << 13;
        event.scrollWheel.fixedDeltaAxis2 = dx << 13;
        self.post_event(os::NX_SCROLLWHEELMOVED, &event, 0, 0)
    }

    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error> {
        let (event_type, button_number) = match button {
            MouseButton::Left => (os::NX_LMOUSEDOWN, 0),
            MouseButton::Right => (os::NX_RMOUSEDOWN, 1),
            MouseButton::Middle => (os::NX_OMOUSEDOWN, 2),
        };
        self.mouse_event(event_type, button_number, true)?;
        self.button_state |= 1 << button_number;
        Ok(())
    }

    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error> {
        let (event_type, button_number) = match button {
            MouseButton::Left => (os::NX_LMOUSEUP, 0),
            MouseButton::Right => (os::NX_RMOUSEUP, 1),
            MouseButton::Middle => (os::NX_OMOUSEUP, 2),
        };
        self.mouse_event(event_type, button_number, false)?;
        self.button_state &= !(1 << button_number);
        Ok(())
    }
}
