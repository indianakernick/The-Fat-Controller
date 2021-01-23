use log::debug;
use enigo::{Enigo, Key, MouseButton, KeyboardControllable, MouseControllable};

#[derive(Debug)]
pub enum EnigoCommand {
    Null,

    MouseMoveTo(i32, i32),
    MouseMoveRelative(i32, i32),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseClick(MouseButton),
    MouseScrollX(i32),
    MouseScrollY(i32),

    KeyDown(Key),
    KeyUp(Key),
    KeyClick(Key),
}

pub fn parse_enigo_command(enigo: &mut Enigo, command: EnigoCommand) {
    debug!("{:?}", command);

    use EnigoCommand::*;
    match command {
        Null => {},

        MouseMoveTo(x, y) => enigo.mouse_move_to(x, y),
        MouseMoveRelative(x, y) => enigo.mouse_move_relative(x, y),
        MouseDown(button) => enigo.mouse_down(button),
        MouseUp(button) => enigo.mouse_up(button),
        MouseClick(button) => enigo.mouse_click(button),
        MouseScrollX(length) => enigo.mouse_scroll_x(length),
        MouseScrollY(length) => enigo.mouse_scroll_y(length),

        KeyDown(key) => enigo.key_down(key),
        KeyUp(key) => enigo.key_up(key),
        KeyClick(key) => enigo.key_click(key),
    }
}
