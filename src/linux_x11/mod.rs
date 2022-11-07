mod ffi;
mod error;
mod keyboard;
mod mouse;
mod screen;

// The implementation of Context::new is adapted from here:
// https://github.com/jordansissel/xdotool/blob/master/xdo.c

use error::PlatformError;
type Error = crate::GenericError<PlatformError>;

#[derive(Copy, Clone)]
struct KeyInfo {
    keysym: ffi::KeySym,
    group: u8,
    modifiers: u8,
    keycode: ffi::KeyCode,
    default: bool,
}

/// The main context used for generating events (Linux-X11).
///
/// The most useful methods are on the [`traits`](crate::traits).
pub struct Context {
    display: *mut ffi::Display,
    screen_number: std::ffi::c_int,
    scroll: crate::linux_common::ScrollAccum,
    key_map: std::collections::HashMap<char, KeyInfo>,
    unused_keycode: ffi::KeyCode,
    modifier_map: *const ffi::XModifierKeymap,
}

unsafe fn no_xtest(display: *mut ffi::Display) -> bool {
    // Passing null pointers for the things we don't need results in a
    // segfault.
    let mut event_base = 0;
    let mut error_base = 0;
    let mut major_version = 0;
    let mut minor_version = 0;
    ffi::XTestQueryExtension(
        display,
        &mut event_base,
        &mut error_base,
        &mut major_version,
        &mut minor_version
    ) == ffi::False
}

unsafe fn find_unused_key_code(
    display: *mut ffi::Display,
    min_keycode: ffi::KeyCode,
    max_keycode: ffi::KeyCode,
) -> Result<ffi::KeyCode, Error> {

    // Get the full mapping from keycodes to keysyms. There may be
    // multiple keysyms for each keycode depending on which modifiers
    // are pressed. We need this for finding an unused keycode, that is
    // a keycode without any associated keysyms.
    let keycode_count = (max_keycode - min_keycode) + 1;
    let mut keysyms_per_keycode = 0;
    let keysyms = ffi::XGetKeyboardMapping(
        display,
        min_keycode,
        keycode_count as std::ffi::c_int,
        &mut keysyms_per_keycode,
    );
    if keysyms.is_null() {
        return Err(Error::Platform(PlatformError::XGetKeyboardMapping));
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
        if slice.iter().all(|keysym| *keysym == ffi::NoSymbol) {
            ffi::XFree(keysyms);
            return Ok(code_idx + min_keycode);
        }
    }

    ffi::XFree(keysyms);
    Err(Error::Platform(PlatformError::NoUnusedKeyCode))
}

unsafe fn create_key_map(
    display: *mut ffi::Display,
    min_keycode: ffi::KeyCode,
    max_keycode: ffi::KeyCode,
) -> Result<std::collections::HashMap<char, KeyInfo>, Error> {

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

    use std::ffi::c_uint;
    use std::collections::hash_map::{HashMap, Entry};

    let desc = ffi::XkbGetMap(display, ffi::XkbAllClientInfoMask, ffi::XkbUseCoreKbd);
    if desc.is_null() {
        return Err(Error::Platform(PlatformError::XkbGetMap));
    }

    let mut key_map = HashMap::new();

    for keycode in min_keycode..=max_keycode {
        let groups = ffi::XkbKeyNumGroups(desc, keycode);
        for group in 0..groups {
            let key_type = ffi::XkbKeyKeyType(desc, keycode, group);
            for level in 0..(*key_type).num_levels {
                let keysym = ffi::XkbKeycodeToKeysym(display, keycode, group as c_uint, level as c_uint);
                let mut modifiers = 0;

                let maps = std::slice::from_raw_parts((*key_type).map, (*key_type).map_count as usize);
                for map in maps {
                    if map.active == ffi::True && map.level == level {
                        modifiers = map.mods.mask;
                        break;
                    }
                }

                let charcode = ffi::xkb_keysym_to_utf32(keysym as ffi::xkb_keysym_t);
                // We only care about keys that yield characters.
                if charcode as u32 == 0 {
                    continue;
                }
                let charcode = match std::char::from_u32(charcode) {
                    Some(c) => c,
                    None => {
                        ffi::XkbFreeClientMap(desc, 0, ffi::True);
                        return Err(Error::Platform(PlatformError::KeySymToUnicode));
                    }
                };

                if let Entry::Vacant(entry) = key_map.entry(charcode) {
                    entry.insert(KeyInfo {
                        keysym,
                        group: group as u8,
                        modifiers,
                        keycode,
                        default: true,
                    });
                }
            }
        }
    }

    ffi::XkbFreeClientMap(desc, 0, ffi::True);

    Ok(key_map)
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let display = ffi::XOpenDisplay(std::ptr::null());
            if display.is_null() {
                return Err(Error::Platform(PlatformError::XOpenDisplay));
            }

            if no_xtest(display) {
                ffi::XCloseDisplay(display);
                return Err(Error::Platform(PlatformError::XTestQueryExtension));
            }

            // Get the full range of keycodes used by X11. This is probably
            // always 8-255 on Linux but we should make sure.
            let mut min_keycode = 0;
            let mut max_keycode = 0;
            ffi::XDisplayKeycodes(display, &mut min_keycode, &mut max_keycode);
            let min_keycode = min_keycode as ffi::KeyCode;
            let max_keycode = max_keycode as ffi::KeyCode;

            let unused_keycode = match find_unused_key_code(display, min_keycode, max_keycode) {
                Ok(k) => k,
                Err(e) => {
                    ffi::XCloseDisplay(display);
                    return Err(e);
                }
            };

            let key_map = match create_key_map(display, min_keycode, max_keycode) {
                Ok(m) => m,
                Err(e) => {
                    ffi::XCloseDisplay(display);
                    return Err(e);
                }
            };

            let modifier_map = ffi::XGetModifierMapping(display);
            if modifier_map.is_null() {
                ffi::XCloseDisplay(display);
                return Err(Error::Platform(PlatformError::XGetModifierMapping));
            }

            Ok(Self {
                display,
                screen_number: ffi::XDefaultScreen(display),
                scroll: Default::default(),
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
            ffi::XChangeKeyboardMapping(
                self.display,
                self.unused_keycode as std::ffi::c_int,
                1,
                &0,
                1,
            );
            ffi::XFreeModifiermap(self.modifier_map);
            ffi::XCloseDisplay(self.display);
        }
    }
}
