use std::{thread, time::Duration};
use crate::{Error, KeyboardContext, MouseContext, Key, MouseButton};

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
    /// Creates a delay for a number of milliseconds
    Delay(u32),
}

impl Command {

    /// Execute a [`Command`](Command) by calling the corresponding method on
    /// [`KeyboardContext`](KeyboardContext) or [`MouseContext`](MouseContext).
    pub fn execute<C>(&self, ctx: &mut C) -> Result<(), Error>
        where C: KeyboardContext + MouseContext
    {
        use Command::*;
        match *self {
            KeyDown(key) => ctx.key_down(key),
            KeyUp(key) => ctx.key_up(key),
            KeyClick(key) => ctx.key_click(key),
            MouseMoveRel(dx, dy) => ctx.mouse_move_rel(dx, dy),
            MouseMoveAbs(x, y) => ctx.mouse_move_abs(x, y),
            MouseWarp(x, y) => ctx.mouse_warp(x, y),
            MouseScroll(dx, dy) => ctx.mouse_scroll(dx, dy),
            MouseDown(button) => ctx.mouse_down(button),
            MouseUp(button) => ctx.mouse_up(button),
            MouseClick(button) => ctx.mouse_click(button),
            Delay(millis) => Ok(thread::sleep(Duration::from_millis(millis as u64))),
        }
    }
}
