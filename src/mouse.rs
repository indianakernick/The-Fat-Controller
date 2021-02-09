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

pub trait MouseContext: crate::FallibleContext {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Self::Error>;
    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Self::Error>;
    fn mouse_warp(&mut self, x: i32, y: i32) -> Result<(), Self::Error>;
    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Self::Error>;
    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Self::Error>;
    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Self::Error>;
    fn mouse_click(&mut self, button: MouseButton) -> Result<(), Self::Error>;
}
