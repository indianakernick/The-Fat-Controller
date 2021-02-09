use crate::Key;

pub trait KeyboardContext: crate::FallibleContext {
    fn key_down(&mut self, key: Key) -> Result<(), Self::Error>;
    fn key_up(&mut self, key: Key) -> Result<(), Self::Error>;
    fn key_click(&mut self, key: Key) -> Result<(), Self::Error>;
}
