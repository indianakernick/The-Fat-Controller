mod os;
mod error;
mod info;
mod keyboard;
mod mouse;
mod unicode_keyboard;

use std::ptr;
use std::os::raw::{c_int, c_uint};
use crate::linux_common::ScrollAccum;

// Largely adapted from here
// https://github.com/jordansissel/xdotool/blob/master/xdo.c

#[derive(Copy, Clone)]
struct KeyInfo {
    keysym: os::KeySym,
    group: u8,
    modifiers: u8,
    keycode: os::KeyCode,
    default: bool,
}

pub use error::Error;

/// The main context used for generating events (Linux-X11).
///
/// The most useful methods are on the [`InfoContext`](crate::InfoContext),
/// [`KeyboardContext`](crate::KeyboardContext) and
/// [`MouseContext`](crate::MouseContext) traits.
pub struct Context {
    display: *mut os::Display,
    screen_number: c_int,
    scroll: ScrollAccum,
    key_map: Vec<(char, KeyInfo)>,
    unused_keycode: os::KeyCode,
    modifier_map: *const os::XModifierKeymap,
}

unsafe fn no_xtest(display: *mut os::Display) -> bool {
    // Passing null pointers for the things we don't need results in a
    // segfault.
    let mut event_base = 0;
    let mut error_base = 0;
    let mut major_version = 0;
    let mut minor_version = 0;
    os::XTestQueryExtension(
        display,
        &mut event_base,
        &mut error_base,
        &mut major_version,
        &mut minor_version
    ) == os::False
}

unsafe fn find_unused_key_code(
    display: *mut os::Display,
    min_keycode: os::KeyCode,
    max_keycode: os::KeyCode,
) -> Result<os::KeyCode, Error> {

    // Get the full mapping from keycodes to keysyms. There may be
    // multiple keysyms for each keycode depending on which modifiers
    // are pressed. We need this for finding an unused keycode, that is
    // a keycode without any associated keysyms.
    let keycode_count = (max_keycode - min_keycode) + 1;
    let mut keysyms_per_keycode = 0;
    let keysyms = os::XGetKeyboardMapping(
        display,
        min_keycode,
        keycode_count as c_int,
        &mut keysyms_per_keycode,
    );
    if keysyms == std::ptr::null() {
        return Err(Error::XGetKeyboardMapping);
    }
    let keysyms_per_keycode = keysyms_per_keycode as usize;

    // Find a keycode that has no keysyms associated with it. This keycode will
    // be used for remapping for the purpose of producing characters that aren't
    // on the default keyboard layout.
    for code_idx in 0..keycode_count {
        let sym_idx = code_idx as usize * keysyms_per_keycode;
        let slice = std::slice::from_raw_parts(
            keysyms.add(sym_idx), keysyms_per_keycode
        );
        if slice.iter().all(|keysym| *keysym == os::NoSymbol) {
            os::XFree(keysyms);
            return Ok(code_idx + min_keycode);
        }
    }

    os::XFree(keysyms);
    Err(Error::NoUnusedKeyCode)
}

unsafe fn create_key_map(
    display: *mut os::Display,
    min_keycode: os::KeyCode,
    max_keycode: os::KeyCode,
) -> Result<Vec<(char, KeyInfo)>, Error> {

    // Fuck, this library is so inconsistent. Sometimes a keycode is a
    // KeyCode and sometimes it's an int. Sometimes a group is an int
    // and sometimes it's an unsigned int or even an unsigned char.
    // Another thing that bothers me is that this seems to have been
    // designed before const was introduced to C.

    // Anyway, the purpose of the code below is to populate the mapping
    // from character codes to keycodes.
    // The list of keysyms associated with a keycode is divided into
    // groups. Each keysym in a group corresponds to a shift level. When
    // sending the key event, the keycode, the group and the modifier
    // key state identify a single keysym.
    // See https://tronche.com/gui/x/xlib/input/keyboard-encoding.html

    let desc = os::XkbGetMap(display, os::XkbAllClientInfoMask, os::XkbUseCoreKbd);
    if desc == std::ptr::null() {
        return Err(Error::XkbGetMap);
    }

    let mut key_map = Vec::new();

    for keycode in min_keycode..=max_keycode {
        let groups = os::XkbKeyNumGroups(desc, keycode);
        for group in 0..groups {
            let key_type = os::XkbKeyKeyType(desc, keycode, group);
            for level in 0..(*key_type).num_levels {
                let keysym = os::XkbKeycodeToKeysym(display, keycode, group as c_uint, level as c_uint);
                let mut modifiers = 0;

                let maps = std::slice::from_raw_parts((*key_type).map, (*key_type).map_count as usize);
                for map in maps {
                    if map.active == os::True && map.level == level {
                        modifiers = map.mods.mask;
                        break;
                    }
                }

                let charcode = os::xkb_keysym_to_utf32(keysym as os::xkb_keysym_t);
                // We only care about keys that yield characters.
                if charcode as u32 == 0 {
                    continue;
                }
                let charcode = match std::char::from_u32(charcode) {
                    Some(c) => c,
                    None => {
                        os::XkbFreeClientMap(desc, 0, os::True);
                        return Err(Error::KeySymToUnicode);
                    }
                };

                key_map.push((charcode, KeyInfo {
                    keysym,
                    group: group as u8,
                    modifiers,
                    keycode,
                    default: true,
                }));
            }
        }
    }

    os::XkbFreeClientMap(desc, 0, os::True);

    // The keymap is sorted by the character code so that we can later do a
    // binary search to find a key. The keymap is likely to have around 100
    // elements (108 when I tested). For such a small number of elements, this
    // is probably faster than a hashmap.
    key_map.sort_by_key(|(c, _)| *c);
    key_map.dedup_by_key(|(c, _)| *c);

    Ok(key_map)
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let display = os::XOpenDisplay(ptr::null());
            if display == ptr::null_mut() {
                return Err(Error::XOpenDisplay);
            }

            if no_xtest(display) {
                os::XCloseDisplay(display);
                return Err(Error::XTestQueryExtension);
            }

            // Get the full range of keycodes used by X11. This is probably
            // always 8-255 on Linux but we should make sure.
            let mut min_keycode = 0;
            let mut max_keycode = 0;
            os::XDisplayKeycodes(display, &mut min_keycode, &mut max_keycode);
            let min_keycode = min_keycode as os::KeyCode;
            let max_keycode = max_keycode as os::KeyCode;

            let unused_keycode = match find_unused_key_code(display, min_keycode, max_keycode) {
                Ok(k) => k,
                Err(e) => {
                    os::XCloseDisplay(display);
                    return Err(e);
                }
            };

            let key_map = match create_key_map(display, min_keycode, max_keycode) {
                Ok(m) => m,
                Err(e) => {
                    os::XCloseDisplay(display);
                    return Err(e);
                }
            };

            let modifier_map = os::XGetModifierMapping(display);
            if modifier_map == std::ptr::null() {
                os::XCloseDisplay(display);
                return Err(Error::XGetModifierMapping);
            }

            Ok(Self {
                display,
                screen_number: os::XDefaultScreen(display),
                scroll: ScrollAccum::default(),
                key_map,
                unused_keycode,
                modifier_map,
            })
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            os::XFreeModifiermap(self.modifier_map);
            os::XCloseDisplay(self.display);
        }
    }
}
