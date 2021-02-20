use crate::Key;
use crate::linux_common;
use std::os::raw::c_uint;
use super::{ffi, Context, Error};

fn key_event(ctx: &Context, key: Key, down: bool) -> Result<(), Error> {
    unsafe {
        let key_code = (linux_common::to_key_code(key) + 8) as c_uint;
        let press = if down { ffi::True } else { ffi::False };
        if ffi::XTestFakeKeyEvent(ctx.display, key_code, press, ffi::CurrentTime) == 0 {
            return Err(Error::XTestFakeKeyEvent);
        }
        ffi::XSync(ctx.display, ffi::False);
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
