use crate::Key;
use super::{os, Context, Error};

// Largely adapted from here
// https://github.com/ccMSC/ckb/blob/master/src/ckb-daemon/input_mac.c

// Some more resources
// https://github.com/unbit/foohid
// https://github.com/VoodooI2C/VoodooI2C
// https://github.com/Siguza/ios-resources
// https://developer.apple.com/library/archive/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/Introduction/Introduction.html#//apple_ref/doc/uid/TP0000011-CH204-TPXREF101
// https://github.com/pqrs-org/Karabiner-Elements/blob/5e39a7a92ad5d858024053daba58bebf59bad5b5/src/vendor/cget/cget/pkg/pqrs-org__cpp-osx-iokit_service_monitor/install/include/pqrs/osx/iokit_service_monitor.hpp

enum KeyCode {
    CapsLock,
    Modifier(u8, u32),
    Regular(u8),
    Media(u8),
}

fn to_key_code(key: Key) -> KeyCode {
    use os::*;
    use Key::*;
    use KeyCode::*;

    match key {
        Key::CapsLock => KeyCode::CapsLock,
        Shift => Modifier(kVK_Shift, NX_DEVICELSHIFTKEYMASK),
        Control => Modifier(kVK_Control, NX_DEVICELCTLKEYMASK),
        Alt => Modifier(kVK_Option, NX_DEVICELALTKEYMASK),
        Meta | ControlOrMeta => Modifier(kVK_Command, NX_DEVICELCMDKEYMASK),
        RightShift => Modifier(kVK_RightShift, NX_DEVICERSHIFTKEYMASK),
        RightControl => Modifier(kVK_RightControl, NX_DEVICERCTLKEYMASK),
        RightAlt => Modifier(kVK_RightOption, NX_DEVICERALTKEYMASK),
        RightMeta | RightControlOrMeta => Modifier(kVK_RightCommand, NX_DEVICERCMDKEYMASK),
        Fn => Modifier(kVK_Function, NX_SECONDARYFNMASK),

        Return => Regular(kVK_Return),
        Escape => Regular(kVK_Escape),
        Delete => Regular(kVK_Delete),
        ForwardDelete => Regular(kVK_ForwardDelete),
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

impl Context {
    fn update_modifiers(&mut self) {
        update_modifiers(&mut self.modifiers, os::NX_DEVICELSHIFTKEYMASK, os::NX_DEVICERSHIFTKEYMASK, os::NX_SHIFTMASK);
        update_modifiers(&mut self.modifiers, os::NX_DEVICELCTLKEYMASK, os::NX_DEVICERCTLKEYMASK, os::NX_CONTROLMASK);
        update_modifiers(&mut self.modifiers, os::NX_DEVICELALTKEYMASK, os::NX_DEVICERALTKEYMASK, os::NX_ALTERNATEMASK);
        update_modifiers(&mut self.modifiers, os::NX_DEVICELCMDKEYMASK, os::NX_DEVICERCMDKEYMASK, os::NX_COMMANDMASK);
    }

    fn key_event(&mut self, key: Key, down: bool) -> Result<(), Error> {
        let event_type = if down { os::NX_KEYDOWN } else { os::NX_KEYUP };
        let mut event = os::NXEventData::default();

        match to_key_code(key) {
            KeyCode::CapsLock => {
                if down {
                    self.modifiers ^= os::NX_ALPHASHIFTMASK;

                    event.key.origCharSet = os::NX_ASCIISET;
                    event.key.charSet = os::NX_ASCIISET;
                    event.key.keyCode = os::kVK_CapsLock as u16;

                    self.post_event(
                        os::NX_FLAGSCHANGED,
                        &event,
                        self.modifiers,
                        os::kIOHIDSetGlobalEventFlags
                    )?;
                }

                event.compound.subType = os::NX_SUBTYPE_AUX_CONTROL_BUTTONS;
                unsafe {
                    event.compound.misc.L[0] = aux_key(os::NX_KEYTYPE_CAPS_LOCK, event_type, false);
                }
                self.post_event(os::NX_SYSDEFINED, &event, 0, 0)
            },

            KeyCode::Modifier(key_code, mask) => {
                if down {
                    self.modifiers |= mask;
                } else {
                    self.modifiers &= !mask;
                }

                self.update_modifiers();

                event.key.origCharSet = os::NX_ASCIISET;
                event.key.charSet = os::NX_ASCIISET;
                event.key.keyCode = key_code as u16;

                self.post_event(
                    os::NX_FLAGSCHANGED,
                    &event,
                    self.modifiers,
                    os::kIOHIDSetGlobalEventFlags
                )
            },

            KeyCode::Regular(key_code) => {
                event.key.origCharSet = os::NX_ASCIISET;
                event.key.charSet = os::NX_ASCIISET;
                event.key.keyCode = key_code as u16;
                self.post_event(event_type, &event, 0, 0)
            },

            KeyCode::Media(key_code) => {
                event.compound.subType = os::NX_SUBTYPE_AUX_CONTROL_BUTTONS;
                unsafe {
                    event.compound.misc.L[0] = aux_key(key_code, event_type, false);
                }
                self.post_event(os::NX_SYSDEFINED, &event, 0, 0)
            },
        }
    }
}

impl crate::KeyboardContext for Context {
    fn key_down(&mut self, key: Key) -> Result<(), Error> {
        self.key_event(key, true)
    }

    fn key_up(&mut self, key: Key) -> Result<(), Error> {
        self.key_event(key, false)
    }
}
