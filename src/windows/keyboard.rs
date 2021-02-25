use crate::Key;
use super::{ffi, Context, Error};

fn to_key_code(key: Key) -> ffi::WORD {
    use Key::*;
    use ffi::*;
    match key {
        CapsLock => VK_CAPITAL,
        Shift => VK_LSHIFT,
        Control | ControlOrMeta => VK_LCONTROL,
        Alt => VK_LMENU,
        Meta => VK_LWIN,
        RightShift => VK_RSHIFT,
        RightControl | RightControlOrMeta => VK_RCONTROL,
        RightAlt => VK_RMENU,
        RightMeta => VK_RWIN,
        // There is no virtual key code for Fn.
        // https://stackoverflow.com/a/48132748/4093378
        Fn => 0xFF,
        ReturnOrEnter => VK_RETURN,
        Escape => VK_ESCAPE,
        DeleteOrBackspace => VK_BACK,
        ForwardDelete => VK_DELETE,
        Tab => VK_TAB,
        Space => VK_SPACE,
        Minus => VK_OEM_MINUS,
        Equal => VK_OEM_PLUS,
        LeftBracket => VK_OEM_4,
        RightBracket => VK_OEM_6,
        Backslash => VK_OEM_5,
        Semicolon => VK_OEM_1,
        Quote => VK_OEM_7,
        Grave => VK_OEM_3,
        Comma => VK_OEM_COMMA,
        Period => VK_OEM_PERIOD,
        Slash => VK_OEM_2,
        UpArrow => VK_UP,
        RightArrow => VK_RIGHT,
        DownArrow => VK_DOWN,
        LeftArrow => VK_LEFT,
        PageUp => VK_PRIOR,
        PageDown => VK_NEXT,
        Home => VK_HOME,
        End => VK_END,
        A => VK_A,
        B => VK_B,
        C => VK_C,
        D => VK_D,
        E => VK_E,
        F => VK_F,
        G => VK_G,
        H => VK_H,
        I => VK_I,
        J => VK_J,
        K => VK_K,
        L => VK_L,
        M => VK_M,
        N => VK_N,
        O => VK_O,
        P => VK_P,
        Q => VK_Q,
        R => VK_R,
        S => VK_S,
        T => VK_T,
        U => VK_U,
        V => VK_V,
        W => VK_W,
        X => VK_X,
        Y => VK_Y,
        Z => VK_Z,
        N0 => VK_0,
        N1 => VK_1,
        N2 => VK_2,
        N3 => VK_3,
        N4 => VK_4,
        N5 => VK_5,
        N6 => VK_6,
        N7 => VK_7,
        N8 => VK_8,
        N9 => VK_9,
        Numpad0 => VK_NUMPAD0,
        Numpad1 => VK_NUMPAD1,
        Numpad2 => VK_NUMPAD2,
        Numpad3 => VK_NUMPAD3,
        Numpad4 => VK_NUMPAD4,
        Numpad5 => VK_NUMPAD5,
        Numpad6 => VK_NUMPAD6,
        Numpad7 => VK_NUMPAD7,
        Numpad8 => VK_NUMPAD8,
        Numpad9 => VK_NUMPAD9,
        // Clear key seems to have no effect.
        NumpadClear => VK_CLEAR,
        NumpadEquals => VK_OEM_PLUS,
        NumpadDivide => VK_DIVIDE,
        NumpadMultiply => VK_MULTIPLY,
        NumpadMinus => VK_OEM_MINUS,
        NumpadPlus => VK_ADD,
        NumpadEnter => VK_RETURN,
        NumpadDecimal => VK_DECIMAL,
        F1 => VK_F1,
        F2 => VK_F2,
        F3 => VK_F3,
        F4 => VK_F4,
        F5 => VK_F5,
        F6 => VK_F6,
        F7 => VK_F7,
        F8 => VK_F8,
        F9 => VK_F9,
        F10 => VK_F10,
        F11 => VK_F11,
        F12 => VK_F12,
        FastForward => VK_MEDIA_NEXT_TRACK,
        Rewind => VK_MEDIA_PREV_TRACK,
        PlayPause => VK_MEDIA_PLAY_PAUSE,
        VolumeUp => VK_VOLUME_UP,
        VolumeDown => VK_VOLUME_DOWN,
        Mute => VK_VOLUME_MUTE,
    }
}

fn key_event(ctx: &Context, key: Key, down: bool) -> Result<(), Error> {
    let mut input = ffi::INPUT::default();
    input.type_ = ffi::INPUT_KEYBOARD;
    input.u.ki.wVk = to_key_code(key);
    input.u.ki.dwFlags = if down { 0 } else { ffi::KEYEVENTF_KEYUP };
    ctx.send_input(&input)
}

impl crate::KeyboardContext for Context {
    fn key_down(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, true)
    }

    fn key_up(&mut self, key: Key) -> Result<(), Error> {
        key_event(self, key, false)
    }

    fn key_click(&mut self, key: Key) -> Result<(), Error> {
        let key_code = to_key_code(key);
        let mut inputs = [ffi::INPUT::default(), ffi::INPUT::default()];
        inputs[0].type_ = ffi::INPUT_KEYBOARD;
        inputs[0].u.ki.wVk = key_code;
        inputs[1].type_ = ffi::INPUT_KEYBOARD;
        inputs[1].u.ki.wVk = key_code;
        inputs[1].u.ki.dwFlags = ffi::KEYEVENTF_KEYUP;
        self.send_inputs(&inputs)
    }
}

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
    fn unicode_char(&mut self, ch: char) -> Result<(), Error> {
        if ch.len_utf16() == 2 {
            return Err(Error::UnsupportedUnicode);
        }
        let state = unsafe {
            ffi::VkKeyScanW(ch as ffi::WCHAR)
        };
        if state == -1 {
            return Err(Error::UnsupportedUnicode);
        }

        char_event(self, state)
    }

    fn unicode_string(&mut self, s: &str) -> Result<(), Error> {
        let mut inputs = s.encode_utf16().map(|code_unit| {
            let mut input = ffi::INPUT::default();
            input.type_ = ffi::INPUT_KEYBOARD;
            input.u.ki.dwFlags = ffi::KEYEVENTF_UNICODE;
            input.u.ki.wScan = code_unit;
            input
        }).collect::<Vec<ffi::INPUT>>();

        self.send_inputs(&inputs)?;

        for input in inputs.iter_mut() {
            input.u.ki.dwFlags = ffi::KEYEVENTF_UNICODE | ffi::KEYEVENTF_KEYUP;
        }

        self.send_inputs(&inputs)
    }
}
