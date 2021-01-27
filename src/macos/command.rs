use log::debug;
use super::{EventContext, Key, Flags, MouseButton};

#[derive(Debug)]
pub enum Command {
    Null,
    MouseMoveTo(i32, i32),
    MouseMoveRelative(i32, i32),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseClick(MouseButton),
    MouseNthClick(MouseButton, u32),
    MouseScrollX(i32),
    MouseScrollY(i32),
    KeyDown(Key),
    KeyUp(Key),
    KeyClick(Key),
    KeyClickFlags(Key, Flags),
}

impl EventContext {
    pub fn evaluate_command(&mut self, command: Command) {
        debug!("{:?}", command);

        use Command::*;

        match command {
            Null => {},
            MouseMoveTo(x, y) => self.mouse_move_to(x, y),
            MouseMoveRelative(x, y) => self.mouse_move_relative(x, y),
            MouseDown(button) => self.mouse_down(button),
            MouseUp(button) => self.mouse_up(button),
            MouseClick(button) => self.mouse_click(button),
            MouseNthClick(button, count) => self.mouse_nth_click(button, count),
            MouseScrollX(length) => self.mouse_scroll_x(length),
            MouseScrollY(length) => self.mouse_scroll_y(length),
            KeyDown(key) => self.key_down(key),
            KeyUp(key) => self.key_up(key),
            KeyClick(key) => self.key_click(key),
            KeyClickFlags(key, flags) => self.key_click_flags(key, flags),
        }
    }
}
