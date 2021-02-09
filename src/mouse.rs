#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    pub const COUNT: u8 = Self::Middle as u8 + 1;
}
