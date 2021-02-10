use crate::{Error, MouseButton};

pub trait MouseContext {
    fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> Result<(), Error>;
    fn mouse_move_abs(&mut self, x: i32, y: i32) -> Result<(), Error>;
    fn mouse_warp(&mut self, x: i32, y: i32) -> Result<(), Error>;
    fn mouse_scroll(&mut self, dx: i32, dy: i32) -> Result<(), Error>;
    fn mouse_down(&mut self, button: MouseButton) -> Result<(), Error>;
    fn mouse_up(&mut self, button: MouseButton) -> Result<(), Error>;
    fn mouse_click(&mut self, button: MouseButton) -> Result<(), Error>;
}
