use super::MouseButton;
use crate::iokit as io;

// Largely adapted from here
// https://github.com/ccMSC/ckb/blob/master/src/ckb-daemon/input_mac.c

impl super::EventContext {
    pub fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> bool {
        let mut event = io::NXEventData::default();
        event.mouseMove.dx = dx;
        event.mouseMove.dy = dy;

        let mut event_type = io::NX_MOUSEMOVED;
        if self.button_state & 0b1 != 0 {
            event_type = io::NX_LMOUSEDRAGGED;
        } else if self.button_state & 0b10 != 0 {
            event_type = io::NX_RMOUSEDRAGGED;
        } else if self.button_state & 0b100 != 0 {
            event_type = io::NX_OMOUSEDRAGGED;
        }

        self.post_event(event_type, &event, io::kIOHIDSetRelativeCursorPosition, 0)
    }

    pub fn mouse_move_abs(&mut self, x: i32, y: i32) -> bool {
        // TODO: get the current mouse position and then do a relative movement
        use std::os::raw::c_int;
        unsafe {
            io::IOHIDSetMouseLocation(
                self.connect,
                x as c_int,
                y as c_int
            ) == io::kIOReturnSuccess
        }
    }

    pub fn mouse_scroll(&mut self, dx: i32, dy: i32) -> bool {
        let mut event = io::NXEventData::default();
        event.scrollWheel.fixedDeltaAxis1 = dy << 13;
        event.scrollWheel.fixedDeltaAxis2 = dx << 13;
        self.post_event(io::NX_SCROLLWHEELMOVED, &event, 0, 0)
    }

    fn mouse_event(&mut self, event_type: u32, click_count: u32, button_number: u8, down: bool) -> bool {
        let mut event = io::NXEventData::default();
        event.compound.subType = io::NX_SUBTYPE_AUX_MOUSE_BUTTONS;
        unsafe {
            event.compound.misc.L[0] = 1 << button_number;
            event.compound.misc.L[1] = if down { 1 << button_number } else { 0 };
        }

        if !self.post_event(io::NX_SYSDEFINED, &event, 0, 0) {
            return false;
        }

        event = io::NXEventData::default();
        event.mouse.click = click_count as i32;
        event.mouse.buttonNumber = button_number;

        self.post_event(event_type, &event, 0, 0)
    }

    pub fn mouse_down(&mut self, button: MouseButton, click_count: u32) -> bool {
        let (event_type, button_number) = match button {
            MouseButton::Left => (io::NX_LMOUSEDOWN, 0),
            MouseButton::Right => (io::NX_RMOUSEDOWN, 1),
            MouseButton::Middle => (io::NX_OMOUSEDOWN, 2),
        };
        if !self.mouse_event(event_type, click_count, button_number, true) {
            return false;
        }
        self.button_state |= 1 << button_number;
        true
    }

    pub fn mouse_up(&mut self, button: MouseButton, click_count: u32) -> bool {
        let (event_type, button_number) = match button {
            MouseButton::Left => (io::NX_LMOUSEUP, 0),
            MouseButton::Right => (io::NX_RMOUSEUP, 1),
            MouseButton::Middle => (io::NX_OMOUSEUP, 2),
        };
        if !self.mouse_event(event_type, click_count, button_number, false) {
            return false;
        }
        self.button_state &= !(1 << button_number);
        true
    }

    pub fn mouse_click(&mut self, button: MouseButton, click_count: u32) -> bool {
        self.mouse_down(button, click_count) && self.mouse_up(button, click_count)
    }
}
