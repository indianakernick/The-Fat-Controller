use crate::enigo_command::EnigoCommand;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum CommandCode {
    MouseMoveTo,
    MouseMoveRelative,
    MouseDown,
    MouseUp,
    MouseClick,
    MouseScrollX,
    MouseScrollY,
    KeyDown,
    KeyUp,
    KeyClick
}

fn parse_int_16(byte_0: u8, byte_1: u8) -> i16 {
    unsafe {
        std::mem::transmute::<[u8; 2], i16>([byte_0, byte_1])
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right
}

fn parse_mouse_button(byte: u8) -> enigo::MouseButton {
    match byte {
        b if b == MouseButton::Left as u8 => enigo::MouseButton::Left,
        b if b == MouseButton::Middle as u8 => enigo::MouseButton::Middle,
        b if b == MouseButton::Right as u8 => enigo::MouseButton::Right,
        _ => panic!("Invalid mouse button: {}", byte)
    }
}

pub fn parse_mouse_button_name(name: &str) -> Option<MouseButton> {
    match name {
        "mouseleft" => Some(MouseButton::Left),
        "mousemiddle" => Some(MouseButton::Middle),
        "mouseright" => Some(MouseButton::Right),
        _ => None
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Key {
    Alt,
    Backspace,
    CapsLock,
    Control,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    Meta,
    Option,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    Shift,
    Space,
    Tab,
    UpArrow,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,

    Tilde,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,
}

fn parse_key(byte: u8) -> enigo::Key {
    match byte {
        b if b == Key::Alt as u8 => enigo::Key::Alt,
        b if b == Key::Backspace as u8 => enigo::Key::Backspace,
        b if b == Key::CapsLock as u8 => enigo::Key::CapsLock,
        b if b == Key::Control as u8 => enigo::Key::Control,
        b if b == Key::Delete as u8 => enigo::Key::Delete,
        b if b == Key::DownArrow as u8 => enigo::Key::DownArrow,
        b if b == Key::End as u8 => enigo::Key::End,
        b if b == Key::Escape as u8 => enigo::Key::Escape,
        b if b == Key::F1 as u8 => enigo::Key::F1,
        b if b == Key::F10 as u8 => enigo::Key::F10,
        b if b == Key::F11 as u8 => enigo::Key::F11,
        b if b == Key::F12 as u8 => enigo::Key::F12,
        b if b == Key::F2 as u8 => enigo::Key::F2,
        b if b == Key::F3 as u8 => enigo::Key::F3,
        b if b == Key::F4 as u8 => enigo::Key::F4,
        b if b == Key::F5 as u8 => enigo::Key::F5,
        b if b == Key::F6 as u8 => enigo::Key::F6,
        b if b == Key::F7 as u8 => enigo::Key::F7,
        b if b == Key::F8 as u8 => enigo::Key::F8,
        b if b == Key::F9 as u8 => enigo::Key::F9,
        b if b == Key::Home as u8 => enigo::Key::Home,
        b if b == Key::LeftArrow as u8 => enigo::Key::LeftArrow,
        b if b == Key::Meta as u8 => enigo::Key::Meta,
        b if b == Key::Option as u8 => enigo::Key::Option,
        b if b == Key::PageDown as u8 => enigo::Key::PageDown,
        b if b == Key::PageUp as u8 => enigo::Key::PageUp,
        b if b == Key::Return as u8 => enigo::Key::Return,
        b if b == Key::RightArrow as u8 => enigo::Key::RightArrow,
        b if b == Key::Shift as u8 => enigo::Key::Shift,
        b if b == Key::Space as u8 => enigo::Key::Space,
        b if b == Key::Tab as u8 => enigo::Key::Tab,
        b if b == Key::UpArrow as u8 => enigo::Key::UpArrow,

        b if b == Key::A as u8 => enigo::Key::Layout('a'),
        b if b == Key::B as u8 => enigo::Key::Layout('b'),
        b if b == Key::C as u8 => enigo::Key::Layout('c'),
        b if b == Key::D as u8 => enigo::Key::Layout('d'),
        b if b == Key::E as u8 => enigo::Key::Layout('e'),
        b if b == Key::F as u8 => enigo::Key::Layout('f'),
        b if b == Key::G as u8 => enigo::Key::Layout('g'),
        b if b == Key::H as u8 => enigo::Key::Layout('h'),
        b if b == Key::I as u8 => enigo::Key::Layout('i'),
        b if b == Key::J as u8 => enigo::Key::Layout('j'),
        b if b == Key::K as u8 => enigo::Key::Layout('k'),
        b if b == Key::L as u8 => enigo::Key::Layout('l'),
        b if b == Key::M as u8 => enigo::Key::Layout('m'),
        b if b == Key::N as u8 => enigo::Key::Layout('n'),
        b if b == Key::O as u8 => enigo::Key::Layout('o'),
        b if b == Key::P as u8 => enigo::Key::Layout('p'),
        b if b == Key::Q as u8 => enigo::Key::Layout('q'),
        b if b == Key::R as u8 => enigo::Key::Layout('r'),
        b if b == Key::S as u8 => enigo::Key::Layout('s'),
        b if b == Key::T as u8 => enigo::Key::Layout('t'),
        b if b == Key::U as u8 => enigo::Key::Layout('u'),
        b if b == Key::V as u8 => enigo::Key::Layout('v'),
        b if b == Key::W as u8 => enigo::Key::Layout('w'),
        b if b == Key::X as u8 => enigo::Key::Layout('x'),
        b if b == Key::Y as u8 => enigo::Key::Layout('y'),
        b if b == Key::Z as u8 => enigo::Key::Layout('z'),

        b if b == Key::N0 as u8 => enigo::Key::Layout('0'),
        b if b == Key::N1 as u8 => enigo::Key::Layout('1'),
        b if b == Key::N2 as u8 => enigo::Key::Layout('2'),
        b if b == Key::N3 as u8 => enigo::Key::Layout('3'),
        b if b == Key::N4 as u8 => enigo::Key::Layout('4'),
        b if b == Key::N5 as u8 => enigo::Key::Layout('5'),
        b if b == Key::N6 as u8 => enigo::Key::Layout('6'),
        b if b == Key::N7 as u8 => enigo::Key::Layout('7'),
        b if b == Key::N8 as u8 => enigo::Key::Layout('8'),
        b if b == Key::N9 as u8 => enigo::Key::Layout('9'),

        b if b == Key::Tilde as u8 => enigo::Key::Layout('~'),
        b if b == Key::Minus as u8 => enigo::Key::Layout('-'),
        b if b == Key::Equal as u8 => enigo::Key::Layout('='),
        b if b == Key::LeftBracket as u8 => enigo::Key::Layout('['),
        b if b == Key::RightBracket as u8 => enigo::Key::Layout(']'),
        b if b == Key::Backslash as u8 => enigo::Key::Layout('\\'),
        b if b == Key::Semicolon as u8 => enigo::Key::Layout(';'),
        b if b == Key::Quote as u8 => enigo::Key::Layout('\''),
        b if b == Key::Comma as u8 => enigo::Key::Layout(','),
        b if b == Key::Period as u8 => enigo::Key::Layout('.'),
        b if b == Key::Slash as u8 => enigo::Key::Layout('/'),

        _ => panic!("Invalid key: {}", byte)
    }
}

pub fn parse_key_name(name: &str) -> Option<Key> {
    match name {
        "alt" => Some(Key::Alt),
        "backspace" => Some(Key::Backspace),
        "capslock" => Some(Key::CapsLock),
        "control" => Some(Key::Control),
        "delete" => Some(Key::Delete),
        "downarrow" => Some(Key::DownArrow),
        "end" => Some(Key::End),
        "escape" => Some(Key::Escape),
        "f1" => Some(Key::F1),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "home" => Some(Key::Home),
        "leftarrow" => Some(Key::LeftArrow),
        "meta" => Some(Key::Meta),
        "option" => Some(Key::Option),
        "pagedown" => Some(Key::PageDown),
        "pageup" => Some(Key::PageUp),
        "return" => Some(Key::Return),
        "rightarrow" => Some(Key::RightArrow),
        "shift" => Some(Key::Shift),
        "space" => Some(Key::Space),
        "tab" => Some(Key::Tab),
        "uparrow" => Some(Key::UpArrow),

        "a" => Some(Key::A),
        "b" => Some(Key::B),
        "c" => Some(Key::C),
        "d" => Some(Key::D),
        "e" => Some(Key::E),
        "f" => Some(Key::F),
        "g" => Some(Key::G),
        "h" => Some(Key::H),
        "i" => Some(Key::I),
        "j" => Some(Key::J),
        "k" => Some(Key::K),
        "l" => Some(Key::L),
        "m" => Some(Key::M),
        "n" => Some(Key::N),
        "o" => Some(Key::O),
        "p" => Some(Key::P),
        "q" => Some(Key::Q),
        "r" => Some(Key::R),
        "s" => Some(Key::S),
        "t" => Some(Key::T),
        "u" => Some(Key::U),
        "v" => Some(Key::V),
        "w" => Some(Key::W),
        "x" => Some(Key::X),
        "y" => Some(Key::Y),
        "z" => Some(Key::Z),

        "0" => Some(Key::N0),
        "1" => Some(Key::N1),
        "2" => Some(Key::N2),
        "3" => Some(Key::N3),
        "4" => Some(Key::N4),
        "5" => Some(Key::N5),
        "6" => Some(Key::N6),
        "7" => Some(Key::N7),
        "8" => Some(Key::N8),
        "9" => Some(Key::N9),

        "tilde" => Some(Key::Tilde),
        "minus" => Some(Key::Minus),
        "equal" => Some(Key::Equal),
        "leftbracket" => Some(Key::LeftBracket),
        "rightbracket" => Some(Key::RightBracket),
        "backslash" => Some(Key::Backslash),
        "semicolon" => Some(Key::Semicolon),
        "quote" => Some(Key::Quote),
        "comma" => Some(Key::Comma),
        "period" => Some(Key::Period),
        "slash" => Some(Key::Slash),

        _ => None
    }
}

pub fn parse_socket_command(buf: &[u8]) -> EnigoCommand {
    if buf.len() == 0 {
        return EnigoCommand::Null;
    }

    match buf[0] {
        b if b == CommandCode::MouseMoveTo as u8 => {
            if buf.len() != 5 {
                panic!("Invalid command: {:?}", buf);
            }
            let x = parse_int_16(buf[1], buf[2]) as i32;
            let y = parse_int_16(buf[3], buf[4]) as i32;
            EnigoCommand::MouseMoveTo(x, y)
        },

        b if b == CommandCode::MouseMoveRelative as u8 => {
            if buf.len() != 5 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::MouseMoveRelative(
                parse_int_16(buf[1], buf[2]) as i32,
                parse_int_16(buf[3], buf[4]) as i32
            )
        },

        b if b == CommandCode::MouseDown as u8 => {
            if buf.len() != 2 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::MouseDown(parse_mouse_button(buf[1]))
        },

        b if b == CommandCode::MouseUp as u8 => {
            if buf.len() != 2 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::MouseUp(parse_mouse_button(buf[1]))
        },

        b if b == CommandCode::MouseClick as u8 => {
            if buf.len() != 2 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::MouseClick(parse_mouse_button(buf[1]))
        },

        b if b == CommandCode::MouseScrollX as u8 => {
            if buf.len() != 3 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::MouseScrollX(parse_int_16(buf[1], buf[2]) as i32)
        },

        b if b == CommandCode::MouseScrollY as u8 => {
            if buf.len() != 3 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::MouseScrollY(parse_int_16(buf[1], buf[2]) as i32)
        },

        b if b == CommandCode::KeyDown as u8 => {
            if buf.len() != 2 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::KeyDown(parse_key(buf[1]))
        },

        b if b == CommandCode::KeyUp as u8 => {
            if buf.len() != 2 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::KeyUp(parse_key(buf[1]))
        },

        b if b == CommandCode::KeyClick as u8 => {
            if buf.len() != 2 {
                panic!("Invalid command: {:?}", buf);
            }
            EnigoCommand::KeyClick(parse_key(buf[1]))
        },

        _ => {
            panic!("Invalid command: {:?}", buf);
        }
    }
}
