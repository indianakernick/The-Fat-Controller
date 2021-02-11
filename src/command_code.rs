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

impl std::str::FromStr for CommandCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CommandCode::*;
        match s {
            "keydown" => Ok(KeyDown),
            "keyup" => Ok(KeyUp),
            "keyclick" => Ok(KeyClick),
            "mousemoverel" => Ok(MouseMoveRel),
            "mousemoveabs" => Ok(MouseMoveAbs),
            "mousewarp" => Ok(MouseWarp),
            "mousescroll" => Ok(MouseScroll),
            "mousedown" => Ok(MouseDown),
            "mouseup" => Ok(MouseUp),
            "mouseclick" => Ok(MouseClick),
            "delay" => Ok(Delay),
            _ => Err(()),
        }
    }
}
