use crate::Error;

pub trait InfoContext {
    fn mouse_location(&self) -> Result<(i32, i32), Error>;
    fn screen_size(&self) -> Result<(i32, i32), Error>;
}
