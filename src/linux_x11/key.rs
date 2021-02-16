use crate::Key;
use crate::linux_common;
use std::os::raw::c_uint;
use super::{os, Context, Error};

impl Context {
    fn key_event(&mut self, key: Key, down: bool) -> Result<(), Error> {
        unsafe {
            let key_code = (linux_common::to_key_code(key) + 8) as c_uint;
            let press = if down { os::True } else { os::False };
            os::XTestFakeKeyEvent(self.display, key_code, press, os::CurrentTime);
            os::XSync(self.display, os::False);
            Ok(())
        }
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
