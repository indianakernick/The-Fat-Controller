use crate::Key;
use crate::linux_common;
use std::os::raw::c_uint;
use super::{os, Context, Error};

fn key_event(ctx: &Context, key: Key, down: bool) -> Result<(), Error> {
    unsafe {
        let key_code = (linux_common::to_key_code(key) + 8) as c_uint;
        let press = if down { os::True } else { os::False };
        if os::XTestFakeKeyEvent(ctx.display, key_code, press, os::CurrentTime) == 0 {
            return Err(Error::XTestFakeKeyEvent);
        }
        os::XSync(ctx.display, os::False);
        Ok(())
    }
}

impl crate::KeyboardContext for Context {
    fn key_down(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, true)
    }

    fn key_up(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, false)
    }
}
