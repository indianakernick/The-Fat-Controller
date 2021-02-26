use crate::{Key, linux_common};
use super::{ffi, Context, Error, KeyInfo, PlatformError};
use std::{thread, time::Duration, os::raw::{c_int, c_uint}};

fn key_event(ctx: &Context, key: Key, down: bool) -> Result<(), Error> {
    unsafe {
        let key_code = (linux_common::to_key_code(key) + 8) as c_uint;
        let press = if down { ffi::True } else { ffi::False };
        if ffi::XTestFakeKeyEvent(ctx.display, key_code, press, ffi::CurrentTime) == 0 {
            return Err(Error::Platform(PlatformError::XTestFakeKeyEvent));
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

// The implementation of UnicodeKeyboardContext is adapted from here:
// https://github.com/jordansissel/xdotool/blob/master/xdo.c

// TODO: Maybe make this configurable
// The delay is only necessary if the layout is changed. However, inserting a
// delay only at the point where the layout changes doesn't work.
const KEY_DELAY: Duration = Duration::from_millis(25);

fn info_from_char(ctx: &Context, ch: char) -> Option<KeyInfo> {
    if let Some(info) = ctx.key_map.get(&ch) {
        return Some(*info);
    }

    let keysym = if ch as u32 >= 0x100 {
        ch as ffi::KeySym + 0x01000000
    } else {
        ch as ffi::KeySym
    };

    unsafe {
        // Checking if the keysym is valid.
        // XKeysymToString returns a pointer to a static string so we're not
        // paying for a memory allocation here.
        if ffi::XKeysymToString(keysym).is_null() {
            return None;
        }
    }

    let modifiers = if ch.is_uppercase() {
        ffi::ShiftMask
    } else {
        0
    };

    // This key is not on the default keyboard layout. This means that the
    // unused keycode will be remapped to this keysym.
    Some(KeyInfo {
        keysym,
        group: 0,
        modifiers,
        keycode: ctx.unused_keycode,
        default: false,
    })
}

unsafe fn modifier_event(ctx: &Context, modifiers: u8, press: ffi::Bool) -> Result<(), Error> {
    // Use the modifier mapping to get the keys associated with a bit in
    // the modifier mask. For each modifier, there may be multiple keys.
    // We press the first non-zero key.

    let key_per_mod = (*ctx.modifier_map).max_keypermod;
    for mod_index in 0..8 {
        if modifiers & (1 << mod_index) == 0 {
            continue;
        }
        for key_index in 0..key_per_mod {
            let index = (mod_index * key_per_mod + key_index) as usize;
            let keycode = *(*ctx.modifier_map).modifiermap.add(index);
            if keycode != 0 {
                if ffi::XTestFakeKeyEvent(ctx.display, keycode as c_uint, press, ffi::CurrentTime) == 0 {
                    return Err(Error::Platform(PlatformError::XTestFakeKeyEvent));
                }
                ffi::XSync(ctx.display, ffi::False);
                break;
            }
        }
    }

    Ok(())
}

unsafe fn key_with_mods_event(ctx: &Context, info: &KeyInfo, down: bool) -> Result<(), Error> {
    // We cannot use XSendEvent here. XSendEvent marks events as fake by
    // setting the send_event property of the XEvent structure. Many
    // applications ignore fake events so we need to use XTestFakeKeyEvent
    // instead.

    // Remember the old group then switch to the new group.
    let old_group = {
        let mut state = std::mem::zeroed();
        ffi::XkbGetState(ctx.display, ffi::XkbUseCoreKbd, &mut state);
        state.group
    };
    if info.group != old_group {
        ffi::XkbLockGroup(ctx.display, ffi::XkbUseCoreKbd, info.group as c_uint);
    }

    // Press the modifiers before.
    if info.modifiers != 0 && down {
        modifier_event(ctx, info.modifiers, ffi::True)?;
    }

    let press = if down { ffi::True } else { ffi::False };
    if ffi::XTestFakeKeyEvent(ctx.display, info.keycode as c_uint, press, ffi::CurrentTime) == 0 {
        return Err(Error::Platform(PlatformError::XTestFakeKeyEvent));
    }

    // Release modifiers after.
    if info.modifiers != 0 && !down {
        modifier_event(ctx, info.modifiers, ffi::False)?;
    }

    // Switching back to the old group now that we're done.
    if info.group != old_group {
        ffi::XkbLockGroup(ctx.display, ffi::XkbUseCoreKbd, old_group as c_uint);
    }

    ffi::XSync(ctx.display, ffi::False);
    thread::sleep(KEY_DELAY);

    Ok(())
}

fn char_event(ctx: &Context, ch: char, down: bool, up: bool) -> Result<(), Error> {
    let info = match info_from_char(ctx, ch) {
        Some(info) => info,
        None => return Err(Error::UnsupportedUnicode(ch)),
    };

    unsafe {
        // If a keysym is not on the default keyboard mapping, we remap the
        // unused keycode.
        if !info.default {
            ffi::XChangeKeyboardMapping(
                ctx.display,
                ctx.unused_keycode as c_int,
                1,
                &info.keysym,
                1,
            );
            ffi::XSync(ctx.display, ffi::False);
        }

        if down {
            key_with_mods_event(ctx, &info, true)?;
        }
        if up {
            key_with_mods_event(ctx, &info, false)?;
        }

        if !info.default {
            ffi::XSync(ctx.display, ffi::False);
        }

        // The keyboard mapping is reset inside Drop.
    }

    Ok(())
}

impl crate::UnicodeKeyboardContext for Context {
    fn unicode_char_down(&mut self, ch: char) -> Result<(), Error> {
        char_event(self, ch, true, false)
    }

    fn unicode_char_up(&mut self, ch: char) -> Result<(), Error> {
        char_event(self, ch, false, true)
    }

    fn unicode_char(&mut self, ch: char) -> Result<(), Error> {
        char_event(self, ch, true, true)
    }

    fn unicode_string(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.chars() {
            if info_from_char(self, ch).is_none() {
                return Err(Error::UnsupportedUnicode(ch));
            }
        }
        for ch in s.chars() {
            self.unicode_char(ch)?;
        }
        Ok(())
    }
}
