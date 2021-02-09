pub trait InfoContext: crate::FallibleContext {
    fn mouse_location(&self) -> Result<(i32, i32), Self::Error>;
    fn screen_size(&self) -> Result<(i32, i32), Self::Error>;
}
