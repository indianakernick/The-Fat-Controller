// Winuser.h

use super::types::*;

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-mouseinput
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct MOUSE_INPUT {
    dx: LONG,
    dy: LONG,
    mouseData: DWORD,
    dwFlags: DWORD,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-keybdinput
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct KEYBD_INPUT {
    wVk: WORD,
    wScan: WORD,
    dwFlags: DWORD,
    time: DWORD,
    dwExtraInfo: ULONG_PTR,
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-hardwareinput
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct HARDWARE_INPUT {
    uMsg: DWORD,
    wParamL: WORD,
    wParamH: WORD,
}

// This is an anonymous union in C
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
union INPUT_UNION {
    mi: MOUSE_INPUT,
    ki: KEYBD_INPUT,
    hi: HARDWARE_INPUT,
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-input
#[repr(C)]
#[derive(Copy, Clone)]
pub struct INPUT {
    r#type: DWORD,
    u: INPUT_UNION,
}

pub type LPINPUT = *mut INPUT;

pub const MOUSEEVENTF_ABSOLUTE: DWORD = 0x8000;
pub const MOUSEEVENTF_HWHEEL: DWORD = 0x1000;
pub const MOUSEEVENTF_MOVE: DWORD = 0x0001;
pub const MOUSEEVENTF_MOVE_NOCOALESCE: DWORD = 0x2000;
pub const MOUSEEVENTF_LEFTDOWN: DWORD = 0x0002;
pub const MOUSEEVENTF_LEFTUP: DWORD = 0x0004;
pub const MOUSEEVENTF_RIGHTDOWN: DWORD = 0x0008;
pub const MOUSEEVENTF_RIGHTUP: DWORD = 0x0010;
pub const MOUSEEVENTF_MIDDLEDOWN: DWORD = 0x0020;
pub const MOUSEEVENTF_MIDDLEUP: DWORD = 0x0040;
pub const MOUSEEVENTF_VIRTUALDESK: DWORD = 0x4000;
pub const MOUSEEVENTF_WHEEL: DWORD = 0x0800;
pub const MOUSEEVENTF_XDOWN: DWORD = 0x0080;
pub const MOUSEEVENTF_XUP: DWORD = 0x0100;

pub const WHEEL_DELTA: DWORD = 120;

pub const XBUTTON1: DWORD = 0x0001;
pub const XBUTTON2: DWORD = 0x0002;

pub const KEYEVENTF_EXTENDEDKEY: DWORD = 0x0001;
pub const KEYEVENTF_KEYUP: DWORD = 0x0002;
pub const KEYEVENTF_SCANCODE: DWORD = 0x0008;
pub const KEYEVENTF_UNICODE: DWORD = 0x0004;

// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

pub const VK_LBUTTON: WORD = 0x01;
pub const VK_RBUTTON: WORD = 0x02;
pub const VK_CANCEL: WORD = 0x03;
pub const VK_MBUTTON: WORD = 0x04;
pub const VK_XBUTTON1: WORD = 0x05;
pub const VK_XBUTTON2: WORD = 0x06;
pub const VK_BACK: WORD = 0x08;
pub const VK_TAB: WORD = 0x09;
pub const VK_CLEAR: WORD = 0x0C;
pub const VK_RETURN: WORD = 0x0D;
pub const VK_SHIFT: WORD = 0x10;
pub const VK_CONTROL: WORD = 0x11;
pub const VK_MENU: WORD = 0x12;
pub const VK_PAUSE: WORD = 0x13;
pub const VK_CAPITAL: WORD = 0x14;
pub const VK_KANA: WORD = 0x15;
pub const VK_HANGUEL: WORD = 0x15;
pub const VK_HANGUL: WORD = 0x15;
pub const VK_IME_ON: WORD = 0x16;
pub const VK_JUNJA: WORD = 0x17;
pub const VK_FINAL: WORD = 0x18;
pub const VK_HANJA: WORD = 0x19;
pub const VK_KANJI: WORD = 0x19;
pub const VK_IME_OFF: WORD = 0x1A;
pub const VK_ESCAPE: WORD = 0x1B;
pub const VK_CONVERT: WORD = 0x1C;
pub const VK_NONCONVERT: WORD = 0x1D;
pub const VK_ACCEPT: WORD = 0x1E;
pub const VK_MODECHANGE: WORD = 0x1F;
pub const VK_SPACE: WORD = 0x20;
pub const VK_PRIOR: WORD = 0x21;
pub const VK_NEXT: WORD = 0x22;
pub const VK_END: WORD = 0x23;
pub const VK_HOME: WORD = 0x24;
pub const VK_LEFT: WORD = 0x25;
pub const VK_UP: WORD = 0x26;
pub const VK_RIGHT: WORD = 0x27;
pub const VK_DOWN: WORD = 0x28;
pub const VK_SELECT: WORD = 0x29;
pub const VK_PRINT: WORD = 0x2A;
pub const VK_EXECUTE: WORD = 0x2B;
pub const VK_SNAPSHOT: WORD = 0x2C;
pub const VK_INSERT: WORD = 0x2D;
pub const VK_DELETE: WORD = 0x2E;
pub const VK_HELP: WORD = 0x2F;
// VK_0 - VK_Z are synthesised
pub const VK_0: WORD = 0x30;
pub const VK_1: WORD = 0x31;
pub const VK_2: WORD = 0x32;
pub const VK_3: WORD = 0x33;
pub const VK_4: WORD = 0x34;
pub const VK_5: WORD = 0x35;
pub const VK_6: WORD = 0x36;
pub const VK_7: WORD = 0x37;
pub const VK_8: WORD = 0x38;
pub const VK_9: WORD = 0x39;
pub const VK_A: WORD = 0x41;
pub const VK_B: WORD = 0x42;
pub const VK_C: WORD = 0x43;
pub const VK_D: WORD = 0x44;
pub const VK_E: WORD = 0x45;
pub const VK_F: WORD = 0x46;
pub const VK_G: WORD = 0x47;
pub const VK_H: WORD = 0x48;
pub const VK_I: WORD = 0x49;
pub const VK_J: WORD = 0x4A;
pub const VK_K: WORD = 0x4B;
pub const VK_L: WORD = 0x4C;
pub const VK_M: WORD = 0x4D;
pub const VK_N: WORD = 0x4E;
pub const VK_O: WORD = 0x4F;
pub const VK_P: WORD = 0x50;
pub const VK_Q: WORD = 0x51;
pub const VK_R: WORD = 0x52;
pub const VK_S: WORD = 0x53;
pub const VK_T: WORD = 0x54;
pub const VK_U: WORD = 0x55;
pub const VK_V: WORD = 0x56;
pub const VK_W: WORD = 0x57;
pub const VK_X: WORD = 0x58;
pub const VK_Y: WORD = 0x59;
pub const VK_Z: WORD = 0x5A;
pub const VK_LWIN: WORD = 0x5B;
pub const VK_RWIN: WORD = 0x5C;
pub const VK_APPS: WORD = 0x5D;
pub const VK_SLEEP: WORD = 0x5F;
pub const VK_NUMPAD0: WORD = 0x60;
pub const VK_NUMPAD1: WORD = 0x61;
pub const VK_NUMPAD2: WORD = 0x62;
pub const VK_NUMPAD3: WORD = 0x63;
pub const VK_NUMPAD4: WORD = 0x64;
pub const VK_NUMPAD5: WORD = 0x65;
pub const VK_NUMPAD6: WORD = 0x66;
pub const VK_NUMPAD7: WORD = 0x67;
pub const VK_NUMPAD8: WORD = 0x68;
pub const VK_NUMPAD9: WORD = 0x69;
pub const VK_MULTIPLY: WORD = 0x6A;
pub const VK_ADD: WORD = 0x6B;
pub const VK_SEPARATOR: WORD = 0x6C;
pub const VK_SUBTRACT: WORD = 0x6D;
pub const VK_DECIMAL: WORD = 0x6E;
pub const VK_DIVIDE: WORD = 0x6F;
pub const VK_F1: WORD = 0x70;
pub const VK_F2: WORD = 0x71;
pub const VK_F3: WORD = 0x72;
pub const VK_F4: WORD = 0x73;
pub const VK_F5: WORD = 0x74;
pub const VK_F6: WORD = 0x75;
pub const VK_F7: WORD = 0x76;
pub const VK_F8: WORD = 0x77;
pub const VK_F9: WORD = 0x78;
pub const VK_F10: WORD = 0x79;
pub const VK_F11: WORD = 0x7A;
pub const VK_F12: WORD = 0x7B;
pub const VK_F13: WORD = 0x7C;
pub const VK_F14: WORD = 0x7D;
pub const VK_F15: WORD = 0x7E;
pub const VK_F16: WORD = 0x7F;
pub const VK_F17: WORD = 0x80;
pub const VK_F18: WORD = 0x81;
pub const VK_F19: WORD = 0x82;
pub const VK_F20: WORD = 0x83;
pub const VK_F21: WORD = 0x84;
pub const VK_F22: WORD = 0x85;
pub const VK_F23: WORD = 0x86;
pub const VK_F24: WORD = 0x87;
pub const VK_NUMLOCK: WORD = 0x90;
pub const VK_SCROLL: WORD = 0x91;
pub const VK_LSHIFT: WORD = 0xA0;
pub const VK_RSHIFT: WORD = 0xA1;
pub const VK_LCONTROL: WORD = 0xA2;
pub const VK_RCONTROL: WORD = 0xA3;
pub const VK_LMENU: WORD = 0xA4;
pub const VK_RMENU: WORD = 0xA5;
pub const VK_BROWSER_BACK: WORD = 0xA6;
pub const VK_BROWSER_FORWARD: WORD = 0xA7;
pub const VK_BROWSER_REFRESH: WORD = 0xA8;
pub const VK_BROWSER_STOP: WORD = 0xA9;
pub const VK_BROWSER_SEARCH: WORD = 0xAA;
pub const VK_BROWSER_FAVORITES: WORD = 0xAB;
pub const VK_BROWSER_HOME: WORD = 0xAC;
pub const VK_VOLUME_MUTE: WORD = 0xAD;
pub const VK_VOLUME_DOWN: WORD = 0xAE;
pub const VK_VOLUME_UP: WORD = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: WORD = 0xB0;
pub const VK_MEDIA_PREV_TRACK: WORD = 0xB1;
pub const VK_MEDIA_STOP: WORD = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: WORD = 0xB3;
pub const VK_LAUNCH_MAIL: WORD = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: WORD = 0xB5;
pub const VK_LAUNCH_APP1: WORD = 0xB6;
pub const VK_LAUNCH_APP2: WORD = 0xB7;
pub const VK_OEM_1: WORD = 0xBA;
pub const VK_OEM_PLUS: WORD = 0xBB;
pub const VK_OEM_COMMA: WORD = 0xBC;
pub const VK_OEM_MINUS: WORD = 0xBD;
pub const VK_OEM_PERIOD: WORD = 0xBE;
pub const VK_OEM_2: WORD = 0xBF;
pub const VK_OEM_3: WORD = 0xC0;
pub const VK_OEM_4: WORD = 0xDB;
pub const VK_OEM_5: WORD = 0xDC;
pub const VK_OEM_6: WORD = 0xDD;
pub const VK_OEM_7: WORD = 0xDE;
pub const VK_OEM_8: WORD = 0xDF;
pub const VK_OEM_102: WORD = 0xE2;
pub const VK_PROCESSKEY: WORD = 0xE5;
pub const VK_PACKET: WORD = 0xE7;
pub const VK_ATTN: WORD = 0xF6;
pub const VK_CRSEL: WORD = 0xF7;
pub const VK_EXSEL: WORD = 0xF8;
pub const VK_EREOF: WORD = 0xF9;
pub const VK_PLAY: WORD = 0xFA;
pub const VK_ZOOM: WORD = 0xFB;
pub const VK_NONAME: WORD = 0xFC;
pub const VK_PA1: WORD = 0xFD;
pub const VK_OEM_CLEAR: WORD = 0xFE;

extern "C" {
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
    #[allow(non_snake_case)]
    pub fn SendInput(
        cInputs: UINT,
        pInputs: LPINPUT,
        cbSize: std::os::raw::c_int,
    ) -> UINT;
}
