use crate::Key;
use unicode_segmentation::UnicodeSegmentation;
use core_graphics::event::{CGEvent, CGEventTapLocation};
use super::{ffi, Context, Error, SHIFT_BIT, OPTION_BIT};

// The implementation of KeyboardContext is adapted from here:
// https://github.com/ccMSC/ckb/blob/master/src/ckb-daemon/input_mac.c

// Some more resources
// https://github.com/unbit/foohid
// https://github.com/VoodooI2C/VoodooI2C
// https://github.com/Siguza/ios-resources
// https://developer.apple.com/library/archive/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/Introduction/Introduction.html#//apple_ref/doc/uid/TP0000011-CH204-TPXREF101
// https://github.com/pqrs-org/Karabiner-Elements/blob/5e39a7a92ad5d858024053daba58bebf59bad5b5/src/vendor/cget/cget/pkg/pqrs-org__cpp-osx-iokit_service_monitor/install/include/pqrs/osx/iokit_service_monitor.hpp

enum GroupedKey {
    CapsLock,
    Modifier(u8, u32),
    Regular(u8),
    Media(u8),
}

fn to_key_code(key: Key) -> GroupedKey {
    use ffi::*;
    use Key::*;
    use GroupedKey::*;

    match key {
        Key::CapsLock => GroupedKey::CapsLock,
        Shift => Modifier(kVK_Shift, NX_DEVICELSHIFTKEYMASK),
        Control => Modifier(kVK_Control, NX_DEVICELCTLKEYMASK),
        Alt => Modifier(kVK_Option, NX_DEVICELALTKEYMASK),
        Meta | ControlOrMeta => Modifier(kVK_Command, NX_DEVICELCMDKEYMASK),
        RightShift => Modifier(kVK_RightShift, NX_DEVICERSHIFTKEYMASK),
        RightControl => Modifier(kVK_RightControl, NX_DEVICERCTLKEYMASK),
        RightAlt => Modifier(kVK_RightOption, NX_DEVICERALTKEYMASK),
        RightMeta | RightControlOrMeta => Modifier(kVK_RightCommand, NX_DEVICERCMDKEYMASK),
        Fn => Modifier(kVK_Function, NX_SECONDARYFNMASK),

        ReturnOrEnter => Regular(kVK_Return),
        Escape => Regular(kVK_Escape),
        DeleteOrBackspace => Regular(kVK_Delete),
        ForwardDelete => Regular(kVK_ForwardDelete),
        Insert => Regular(0xFF),
        Tab => Regular(kVK_Tab),
        Space => Regular(kVK_Space),
        Minus => Regular(kVK_ANSI_Minus),
        Equal => Regular(kVK_ANSI_Equal),
        LeftBracket => Regular(kVK_ANSI_LeftBracket),
        RightBracket => Regular(kVK_ANSI_RightBracket),
        Backslash => Regular(kVK_ANSI_Backslash),
        Semicolon => Regular(kVK_ANSI_Semicolon),
        Quote => Regular(kVK_ANSI_Quote),
        Grave => Regular(kVK_ANSI_Grave),
        Comma => Regular(kVK_ANSI_Comma),
        Period => Regular(kVK_ANSI_Period),
        Slash => Regular(kVK_ANSI_Slash),

        UpArrow => Regular(kVK_UpArrow),
        RightArrow => Regular(kVK_RightArrow),
        DownArrow => Regular(kVK_DownArrow),
        LeftArrow => Regular(kVK_LeftArrow),
        PageUp => Regular(kVK_PageUp),
        PageDown => Regular(kVK_PageDown),
        Home => Regular(kVK_Home),
        End => Regular(kVK_End),

        A => Regular(kVK_ANSI_A),
        B => Regular(kVK_ANSI_B),
        C => Regular(kVK_ANSI_C),
        D => Regular(kVK_ANSI_D),
        E => Regular(kVK_ANSI_E),
        F => Regular(kVK_ANSI_F),
        G => Regular(kVK_ANSI_G),
        H => Regular(kVK_ANSI_H),
        I => Regular(kVK_ANSI_I),
        J => Regular(kVK_ANSI_J),
        K => Regular(kVK_ANSI_K),
        L => Regular(kVK_ANSI_L),
        M => Regular(kVK_ANSI_M),
        N => Regular(kVK_ANSI_N),
        O => Regular(kVK_ANSI_O),
        P => Regular(kVK_ANSI_P),
        Q => Regular(kVK_ANSI_Q),
        R => Regular(kVK_ANSI_R),
        S => Regular(kVK_ANSI_S),
        T => Regular(kVK_ANSI_T),
        U => Regular(kVK_ANSI_U),
        V => Regular(kVK_ANSI_V),
        W => Regular(kVK_ANSI_W),
        X => Regular(kVK_ANSI_X),
        Y => Regular(kVK_ANSI_Y),
        Z => Regular(kVK_ANSI_Z),

        N0 => Regular(kVK_ANSI_0),
        N1 => Regular(kVK_ANSI_1),
        N2 => Regular(kVK_ANSI_2),
        N3 => Regular(kVK_ANSI_3),
        N4 => Regular(kVK_ANSI_4),
        N5 => Regular(kVK_ANSI_5),
        N6 => Regular(kVK_ANSI_6),
        N7 => Regular(kVK_ANSI_7),
        N8 => Regular(kVK_ANSI_8),
        N9 => Regular(kVK_ANSI_9),

        Numpad0 => Regular(kVK_ANSI_Keypad0),
        Numpad1 => Regular(kVK_ANSI_Keypad1),
        Numpad2 => Regular(kVK_ANSI_Keypad2),
        Numpad3 => Regular(kVK_ANSI_Keypad3),
        Numpad4 => Regular(kVK_ANSI_Keypad4),
        Numpad5 => Regular(kVK_ANSI_Keypad5),
        Numpad6 => Regular(kVK_ANSI_Keypad6),
        Numpad7 => Regular(kVK_ANSI_Keypad7),
        Numpad8 => Regular(kVK_ANSI_Keypad8),
        Numpad9 => Regular(kVK_ANSI_Keypad9),

        NumpadClear => Regular(kVK_ANSI_KeypadClear),
        NumpadEquals => Regular(kVK_ANSI_KeypadEquals),
        NumpadDivide => Regular(kVK_ANSI_KeypadDivide),
        NumpadMultiply => Regular(kVK_ANSI_KeypadMultiply),
        NumpadMinus => Regular(kVK_ANSI_KeypadMinus),
        NumpadPlus => Regular(kVK_ANSI_KeypadPlus),
        NumpadEnter => Regular(kVK_ANSI_KeypadEnter),
        NumpadDecimal => Regular(kVK_ANSI_KeypadDecimal),

        F1 => Regular(kVK_F1),
        F2 => Regular(kVK_F2),
        F3 => Regular(kVK_F3),
        F4 => Regular(kVK_F4),
        F5 => Regular(kVK_F5),
        F6 => Regular(kVK_F6),
        F7 => Regular(kVK_F7),
        F8 => Regular(kVK_F8),
        F9 => Regular(kVK_F9),
        F10 => Regular(kVK_F10),
        F11 => Regular(kVK_F11),
        F12 => Regular(kVK_F12),

        FastForward => Media(NX_KEYTYPE_FAST), // FAST and NEXT seem to be the same
        Rewind => Media(NX_KEYTYPE_REWIND), // REWIND and PREVIOUS seem to be the same
        PlayPause => Media(NX_KEYTYPE_PLAY),
        VolumeUp => Media(NX_KEYTYPE_SOUND_UP),
        VolumeDown => Media(NX_KEYTYPE_SOUND_DOWN),
        Mute => Media(NX_KEYTYPE_MUTE),
    }
}

fn aux_key(key_code: u8, event_type: u32, repeat: bool) -> i32 {
    (((key_code as u32) << 16) | (event_type << 8) | (repeat as u32)) as i32
}

fn update_modifiers(modifiers: &mut u32, left: u32, right: u32, both: u32) {
    if *modifiers & left != 0 || *modifiers & right != 0 {
        *modifiers |= both;
    } else {
        *modifiers &= !both;
    }
}

fn update_context_modifiers(ctx: &mut Context) {
    update_modifiers(&mut ctx.modifiers, ffi::NX_DEVICELSHIFTKEYMASK, ffi::NX_DEVICERSHIFTKEYMASK, ffi::NX_SHIFTMASK);
    update_modifiers(&mut ctx.modifiers, ffi::NX_DEVICELCTLKEYMASK, ffi::NX_DEVICERCTLKEYMASK, ffi::NX_CONTROLMASK);
    update_modifiers(&mut ctx.modifiers, ffi::NX_DEVICELALTKEYMASK, ffi::NX_DEVICERALTKEYMASK, ffi::NX_ALTERNATEMASK);
    update_modifiers(&mut ctx.modifiers, ffi::NX_DEVICELCMDKEYMASK, ffi::NX_DEVICERCMDKEYMASK, ffi::NX_COMMANDMASK);
}

fn flags_event(ctx: &Context, event: *const ffi::NXEventData) -> Result<(), Error> {
    ctx.post_event(
        ffi::NX_FLAGSCHANGED,
        event,
        ctx.modifiers,
        ffi::kIOHIDSetGlobalEventFlags
    )
}

fn modifier_key_event(
    ctx: &mut Context,
    event: &mut ffi::NXEventData,
    key_code: u8,
    mask: u32,
    down: bool,
) -> Result<(), Error> {
    if down {
        ctx.modifiers |= mask;
    } else {
        ctx.modifiers &= !mask;
    }
    update_context_modifiers(ctx);
    event.key.keyCode = key_code as u16;
    flags_event(ctx, event)
}

fn key_event(ctx: &mut Context, key: Key, down: bool) -> Result<(), Error> {
    let event_type = if down { ffi::NX_KEYDOWN } else { ffi::NX_KEYUP };
    let mut event = ffi::NXEventData::default();

    match to_key_code(key) {
        GroupedKey::CapsLock => {
            if down {
                ctx.modifiers ^= ffi::NX_ALPHASHIFTMASK;
                event.key.keyCode = ffi::kVK_CapsLock as u16;
                flags_event(ctx, &event)?;
            }

            event.compound.subType = ffi::NX_SUBTYPE_AUX_CONTROL_BUTTONS;
            unsafe {
                event.compound.misc.L[0] = aux_key(ffi::NX_KEYTYPE_CAPS_LOCK, event_type, false);
            }
            ctx.post_event(ffi::NX_SYSDEFINED, &event, 0, 0)
        }

        GroupedKey::Modifier(key_code, mask) => {
            modifier_key_event(ctx, &mut event, key_code, mask, down)
        }

        GroupedKey::Regular(key_code) => {
            event.key.keyCode = key_code as u16;
            ctx.post_event(event_type, &event, 0, 0)
        }

        GroupedKey::Media(key_code) => {
            event.compound.subType = ffi::NX_SUBTYPE_AUX_CONTROL_BUTTONS;
            unsafe {
                event.compound.misc.L[0] = aux_key(key_code, event_type, false);
            }
            ctx.post_event(ffi::NX_SYSDEFINED, &event, 0, 0)
        }
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

fn char_event(ctx: &mut Context, ch: char, down: bool, up: bool) -> Result<(), Error> {
    let info = match ctx.key_map.get(&ch) {
        Some(info) => *info,
        None => return Err(Error::UnsupportedUnicode(ch)),
    };
    let mut event = ffi::NXEventData::default();

    if down {
        if info.modifiers & (1 << SHIFT_BIT) != 0 {
            modifier_key_event(ctx, &mut event, ffi::kVK_Shift, ffi::NX_DEVICELSHIFTKEYMASK, true)?;
        }
        if info.modifiers & (1 << OPTION_BIT) != 0 {
            modifier_key_event(ctx, &mut event, ffi::kVK_Option, ffi::NX_DEVICELALTKEYMASK, true)?;
        }

        event.key.keyCode = info.key_code as u16;
        ctx.post_event(ffi::NX_KEYDOWN, &event, 0, 0)?;
    }

    if up {
        event.key.keyCode = info.key_code as u16;
        ctx.post_event(ffi::NX_KEYUP, &event, 0, 0)?;

        if info.modifiers & (1 << OPTION_BIT) != 0 {
            modifier_key_event(ctx, &mut event, ffi::kVK_Option, ffi::NX_DEVICELALTKEYMASK, false)?;
        }
        if info.modifiers & (1 << SHIFT_BIT) != 0 {
            modifier_key_event(ctx, &mut event, ffi::kVK_Shift, ffi::NX_DEVICELSHIFTKEYMASK, false)?;
        }
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
        // CGEventKeyboardSetUnicodeString only handles the first 20 UTF-16 code
        // units (in other words, 40 bytes) and ignores the rest so we need to
        // split the string up. Also, special characters like tab, line-feed and
        // backspace work in TextEdit but not CLion. Maybe create a special case
        // for these?

        let event = match CGEvent::new_keyboard_event(self.event_source.clone(), 0, true) {
            Ok(e) => e,
            Err(()) => return Err(Error::Unknown),
        };
        for grapheme in s.graphemes(true) {
            event.set_string(grapheme);
            event.post(CGEventTapLocation::HID);
        }

        Ok(())
    }
}
