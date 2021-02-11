// This file was generated automatically

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    pub const COUNT: u8 = 3;
}

impl std::str::FromStr for MouseButton {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MouseButton::*;
        match s {
            "left" => Ok(Left),
            "right" => Ok(Right),
            "middle" => Ok(Middle),
            _ => Err(()),
        }
    }
}
