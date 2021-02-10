use crate::{Error, KeyboardContext, MouseContext, Context, Key, MouseButton};

/// A future invocation of a method on a [`Context`](Context).
///
/// Commands can be executed by calling
/// [`execute_command`](Context::execute_command).
/// Each variant corresponds to a method on either the
/// [`KeyboardContext`](KeyboardContext) or [`MouseContext`](MouseContext)
/// traits.
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    /// Corresponds to [`key_down`](KeyboardContext::key_down)
    KeyDown(Key),
    /// Corresponds to [`key_up`](KeyboardContext::key_up)
    KeyUp(Key),
    /// Corresponds to [`key_click`](KeyboardContext::key_click)
    KeyClick(Key),
    /// Corresponds to [`mouse_move_rel`](MouseContext::mouse_move_rel)
    MouseMoveRel(i32, i32),
    /// Corresponds to [`mouse_move_abs`](MouseContext::mouse_move_abs)
    MouseMoveAbs(i32, i32),
    /// Corresponds to [`mouse_warp`](MouseContext::mouse_warp)
    MouseWarp(i32, i32),
    /// Corresponds to [`mouse_scroll`](MouseContext::mouse_scroll)
    MouseScroll(i32, i32),
    /// Corresponds to [`mouse_down`](MouseContext::mouse_down)
    MouseDown(MouseButton),
    /// Corresponds to [`mouse_up`](MouseContext::mouse_up)
    MouseUp(MouseButton),
    /// Corresponds to [`mouse_click`](MouseContext::mouse_click)
    MouseClick(MouseButton),
}

impl Context {

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// [`KeyboardContext`](KeyboardContext) or [`MouseContext`](MouseContext).
    pub fn execute_command(&mut self, command: Command) -> Result<(), Error> {
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
