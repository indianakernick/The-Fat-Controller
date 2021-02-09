use super::{Context, Key, MouseButton};

#[derive(Debug)]
pub enum Command {
    KeyDown(Key),
    KeyUp(Key),
    KeyClick(Key),
    MouseMoveRel(i32, i32),
    MouseMoveAbs(i32, i32),
    MouseWarp(i32, i32),
    MouseScroll(i32, i32),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseClick(MouseButton),
}

// Do we need a delay command?
// Should we use 16 bit integers here since they're used in the byte representation?
// Don't need click count!
// Maybe don't need key repeat either then?

impl Context {
    pub fn execute_command(&mut self, command: Command) -> bool {
        use Command::*;
        match command {
            KeyDown(key) => self.key_down(key),
            KeyUp(key) => self.key_up(key),
            KeyClick(key) => self.key_click(key),
            MouseMoveRel(dx, dy) => self.mouse_move_rel(dx, dy),
            MouseMoveAbs(x, y) => self.mouse_move_abs(x, y),
            MouseWarp(x, y) => self.mouse_warp(x, y),
            MouseScroll(dx, dy) => self.mouse_scroll(dx, dy),
            MouseDown(button) => self.mouse_down(button),
            MouseUp(button) => self.mouse_up(button),
            MouseClick(button) => self.mouse_click(button),
        }
    }
}
