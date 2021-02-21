use crate::Error;
use super::{Context, ffi};

fn char_event(ctx: &Context, state: ffi::SHORT) -> Result<(), Error> {
    let key = (state & 0xFF) as ffi::WORD;
    let shift = state & (1 << 8) != 0;
    let control = state & (1 << 9) != 0;
    let alt = state & (1 << 10) != 0;
    // There's another bit for a Hankaku key but there doesn't seem to be a
    // key code for it.
    // let hankaku = state & (1 << 11) != 0;

    let mut input = ffi::INPUT::default();
    input.type_ = ffi::INPUT_KEYBOARD;

    if shift {
        input.u.ki.wVk = ffi::VK_LSHIFT;
        ctx.send_input(&input)?;
    }
    if control {
        input.u.ki.wVk = ffi::VK_LCONTROL;
        ctx.send_input(&input)?;
    }
    if alt {
        input.u.ki.wVk = ffi::VK_LMENU;
        ctx.send_input(&input)?;
    }

    input.u.ki.wVk = key;
    ctx.send_input(&input)?;
    input.u.ki.dwFlags = ffi::KEYEVENTF_KEYUP;
    ctx.send_input(&input)?;

    if alt {
        input.u.ki.wVk = ffi::VK_LMENU;
        ctx.send_input(&input)?;
    }
    if control {
        input.u.ki.wVk = ffi::VK_LCONTROL;
        ctx.send_input(&input)?;
    }
    if shift {
        input.u.ki.wVk = ffi::VK_LSHIFT;
        ctx.send_input(&input)?;
    }

    Ok(())
}

impl crate::UnicodeKeyboardContext for Context {
    fn unicode_char(&mut self, ch: char) -> Option<Result<(), Error>> {
        if ch.len_utf16() == 2 {
            return None;
        }
        let state = unsafe {
            ffi::VkKeyScanW(ch as ffi::WCHAR)
        };
        if state == -1 {
            return None;
        }

        Some(char_event(self, state))
    }

    fn unicode_string(&mut self, s: &str) -> Option<Result<(), Error>> {
        let mut inputs = s.encode_utf16().map(|code_unit| {
            let mut input = ffi::INPUT::default();
            input.type_ = ffi::INPUT_KEYBOARD;
            input.u.ki.dwFlags = ffi::KEYEVENTF_UNICODE;
            input.u.ki.wScan = code_unit;
            input
        }).collect::<Vec<ffi::INPUT>>();
        let count = inputs.len() as ffi::UINT;

        unsafe {
            if ffi::SendInput(count, inputs.as_ptr(), ffi::SIZEOF_INPUT) != count {
                return Some(Err(Error::last()));
            }
        }

        for input in inputs.iter_mut() {
            input.u.ki.dwFlags = ffi::KEYEVENTF_UNICODE | ffi::KEYEVENTF_KEYUP;
        }

        unsafe {
            if ffi::SendInput(count, inputs.as_ptr(), ffi::SIZEOF_INPUT) != count {
                return Some(Err(Error::last()));
            }
        }

        Some(Ok(()))
    }
}
