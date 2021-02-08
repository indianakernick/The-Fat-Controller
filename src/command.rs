use super::{EventContext, Key, MouseButton};

pub enum Command {
    Null,
    KeyDown(Key, bool),
    KeyUp(Key),
    KeyClick(Key),
    MouseMoveRel(i32, i32),
    MouseMoveAbs(i32, i32),
    MouseWarp(i32, i32),
    MouseScroll(i32, i32),
    MouseDown(MouseButton, u32),
    MouseUp(MouseButton, u32),
    MouseClick(MouseButton, u32),
}

impl EventContext {
    pub fn execute_command(&mut self, command: Command) -> bool {
        use Command::*;
        match command {
            Null => true,
            KeyDown(key, repeat) => self.key_down(key, repeat),
            KeyUp(key) => self.key_up(key),
            KeyClick(key) => self.key_click(key),
            MouseMoveRel(dx, dy) => self.mouse_move_rel(dx, dy),
            MouseMoveAbs(x, y) => self.mouse_move_abs(x, y),
            MouseWarp(x, y) => self.mouse_warp(x, y),
            MouseScroll(dx, dy) => self.mouse_scroll(dx, dy),
            MouseDown(button, click_count) => self.mouse_down(button, click_count),
            MouseUp(button, click_count) => self.mouse_up(button, click_count),
            MouseClick(button, click_count) => self.mouse_click(button, click_count),
        }
    }
}
