// This file was generated automatically

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandCode {
    Delay,
    KeyDown,
    KeyUp,
    KeyClick,
    MouseMoveRel,
    MouseMoveAbs,
    MouseScroll,
    MouseDown,
    MouseUp,
    MouseClick,
    AsciiCharDown,
    AsciiCharUp,
    AsciiChar,
    AsciiString,
    UnicodeCharDown,
    UnicodeCharUp,
    UnicodeChar,
    UnicodeString,
}

impl CommandCode {
    pub const COUNT: u8 = 18;
}

impl std::str::FromStr for CommandCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CommandCode::*;
        match s {
            "delay" => Ok(Delay),
            "keydown" => Ok(KeyDown),
            "keyup" => Ok(KeyUp),
            "keyclick" => Ok(KeyClick),
            "mousemoverel" => Ok(MouseMoveRel),
            "mousemoveabs" => Ok(MouseMoveAbs),
            "mousescroll" => Ok(MouseScroll),
            "mousedown" => Ok(MouseDown),
            "mouseup" => Ok(MouseUp),
            "mouseclick" => Ok(MouseClick),
            "asciichardown" => Ok(AsciiCharDown),
            "asciicharup" => Ok(AsciiCharUp),
            "asciichar" => Ok(AsciiChar),
            "asciistring" => Ok(AsciiString),
            "unicodechardown" => Ok(UnicodeCharDown),
            "unicodecharup" => Ok(UnicodeCharUp),
            "unicodechar" => Ok(UnicodeChar),
            "unicodestring" => Ok(UnicodeString),
            _ => Err(()),
        }
    }
}
