use crate::Key;
use crate::linux_common;
use super::{os, Context, Error};

impl Context {
    fn key_event(&mut self, key: Key, down: bool) -> Result<(), Error> {
        self.write(os::EV_KEY, linux_common::to_key_code(key), if down { 1 } else { 0 })?;
        self.write_syn_report()
    }
}

impl crate::KeyboardContext for Context {
    fn key_down(&mut self, key: Key) -> Result<(), Error> {
        self.key_event(key, true)
    }

    fn key_up(&mut self, key: Key) -> Result<(), Error> {
        self.key_event(key, false)
    }
}
