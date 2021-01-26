use std::convert::TryFrom;
use crate::macos::{Command, CommandCode, Key, MouseButton};

fn parse_int_16(byte_0: u8, byte_1: u8) -> i16 {
    ((byte_0 as i16) << 8) | (byte_1 as i16)
}

fn parse_mouse_button(byte: u8) -> MouseButton {
    match MouseButton::try_from(byte) {
        Ok(button) => button,
        Err(_) => panic!("Invalid mouse button: {}", byte),
    }
}

fn parse_key(byte: u8) -> Key {
    match Key::try_from(byte) {
        Ok(key) => key,
        Err(_) => panic!("Invalid key: {}", byte),
    }
}

pub fn parse_socket_command(buf: &[u8]) -> (Command, usize) {
    if buf.len() == 0 {
        return (Command::Null, 0);
    }

    let code = match CommandCode::try_from(buf[0]) {
        Ok(code) => code,
        Err(_) => panic!("Invalid command: {:?}", buf),
    };

    match code {
        CommandCode::MouseMoveTo => {
            if buf.len() < 5 {
                panic!("Invalid command: {:?}", buf);
            }
            let x = parse_int_16(buf[1], buf[2]) as i32;
            let y = parse_int_16(buf[3], buf[4]) as i32;
            (Command::MouseMoveTo(x, y), 5)
        },

        CommandCode::MouseMoveRelative => {
            if buf.len() < 5 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseMoveRelative(
                parse_int_16(buf[1], buf[2]) as i32,
                parse_int_16(buf[3], buf[4]) as i32
            ), 5)
        },

        CommandCode::MouseDown => {
            if buf.len() < 2 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseDown(parse_mouse_button(buf[1])), 2)
        },

        CommandCode::MouseUp => {
            if buf.len() < 2 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseUp(parse_mouse_button(buf[1])), 2)
        },

        CommandCode::MouseClick => {
            if buf.len() < 2 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseClick(parse_mouse_button(buf[1])), 2)
        },

        CommandCode::MouseNthClick => {
            if buf.len() < 3 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseNthClick(parse_mouse_button(buf[1]), buf[2] as u32), 3)
        },

        CommandCode::MouseScrollX => {
            if buf.len() < 3 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseScrollX(parse_int_16(buf[1], buf[2]) as i32), 3)
        },

        CommandCode::MouseScrollY => {
            if buf.len() < 3 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::MouseScrollY(parse_int_16(buf[1], buf[2]) as i32), 3)
        },

        CommandCode::KeyDown => {
            if buf.len() < 2 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::KeyDown(parse_key(buf[1])), 2)
        },

        CommandCode::KeyUp => {
            if buf.len() < 2 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::KeyUp(parse_key(buf[1])), 2)
        },

        CommandCode::KeyClick => {
            if buf.len() < 2 {
                panic!("Invalid command: {:?}", buf);
            }
            (Command::KeyClick(parse_key(buf[1])), 2)
        },
    }
}
