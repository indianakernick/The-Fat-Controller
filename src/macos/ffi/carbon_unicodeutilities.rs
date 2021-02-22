// CarbonCore/UnicodeUtilities.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/Headers/UnicodeUtilities.h

use super::{OptionBits, OSStatus, UniChar, UniCharCount};

#[repr(transparent)]
pub struct UCKeyboardLayout(std::ffi::c_void);

#[allow(non_upper_case_globals)]
pub const kUCKeyActionDisplay: u16 = 3;
#[allow(non_upper_case_globals)]
pub const kUCKeyTranslateNoDeadKeysMask: OptionBits = 1;

#[link(name = "Carbon", kind = "framework")]
extern {
    #[allow(non_snake_case)]
    pub fn UCKeyTranslate(
        keyLayoutPtr: *const UCKeyboardLayout,
        virtualKeyCode: u16,
        keyAction: u16,
        modifierKeyState: u32,
        keyboardType: u32,
        keyTranslateOptions: OptionBits,
        deadKeyState: *mut u32,
        maxStringLength: UniCharCount,
        actualStringLength: *mut UniCharCount,
        unicodeString: *mut UniChar,
    ) -> OSStatus;
}
