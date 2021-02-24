use crate::{Key, linux_common};
use super::{ffi, Context, Error};

fn key_event(ctx: &Context, key: Key, down: bool) -> Result<(), Error> {
    ctx.write(ffi::EV_KEY, linux_common::to_key_code(key), if down { 1 } else { 0 })?;
    ctx.write_syn_report()
}

impl crate::KeyboardContext for Context {
    fn key_down(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, true)
    }

    fn key_up(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, false)
    }
}
