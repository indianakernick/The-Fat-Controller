use std::thread;
use std::time::Duration;
use std::os::raw::{c_int, c_uint};
use super::{ffi, Context, Error, KeyInfo};

// Largely adapted from here
// https://github.com/jordansissel/xdotool/blob/master/xdo.c

// TODO: Maybe make this configurable
const KEY_DELAY: Duration = Duration::from_millis(25);

fn modifiers_from_char(ch: char) -> u8 {
    if ch.is_uppercase() {
        ffi::ShiftMask
    } else {
        0
    }
}

fn info_from_char(ctx: &Context, ch: char) -> Option<KeyInfo> {
    if let Ok(index) = ctx.key_map.binary_search_by_key(&ch, |(c, _)| *c) {
        return Some(ctx.key_map[index].1);
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
        if ffi::XKeysymToString(keysym) == std::ptr::null() {
            return None;
        }
    }

    // This key is not on the default keyboard layout. This means that the
    // unused keycode will be remapped to this keysym.
    Some(KeyInfo {
        keysym,
        group: 0,
        modifiers: modifiers_from_char(ch),
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
                    return Err(Error::XTestFakeKeyEvent);
                }
                ffi::XSync(ctx.display, ffi::False);
                break;
            }
        }
    }

    Ok(())
}

unsafe fn key_event(ctx: &Context, info: &KeyInfo, down: bool) -> Result<(), Error> {
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
        return Err(Error::XTestFakeKeyEvent);
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

unsafe fn char_event(ctx: &Context, info: KeyInfo) -> Result<(), Error> {
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

    key_event(ctx, &info, true)?;
    key_event(ctx, &info, false)?;

    if !info.default {
        ffi::XSync(ctx.display, ffi::False);
    }

    // The keyboard mapping might have changed by this point but there's no
    // need to worry about it. It's not going to affect anything other than
    // this function.

    Ok(())
}

impl crate::UnicodeKeyboardContext for Context {
    fn unicode_char(&mut self, ch: char) -> Option<Result<(), Error>> {
        let info = match info_from_char(self, ch) {
            Some(info) => info,
            None => return None,
        };
        unsafe {
            Some(char_event(self, info))
        }
    }

    fn unicode_string(&mut self, s: &str) -> Option<Result<(), Error>> {
        for ch in s.chars() {
            match self.unicode_char(ch) {
                Some(Ok(_)) => continue,
                err => return err,
            }
        }
        Some(Ok(()))
    }
}
