use crate::{Key, Error};

pub trait KeyboardContext {
    fn key_down(&mut self, key: Key) -> Result<(), Error>;
    fn key_up(&mut self, key: Key) -> Result<(), Error>;
    fn key_click(&mut self, key: Key) -> Result<(), Error>;
}
