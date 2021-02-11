// This file was generated automatically

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandCode {
    KeyDown,
    KeyUp,
    KeyClick,
    MouseMoveRel,
    MouseMoveAbs,
    MouseWarp,
    MouseScroll,
    MouseDown,
    MouseUp,
    MouseClick,
    Delay,
}

impl CommandCode {
    pub const COUNT: u8 = 11;
}
