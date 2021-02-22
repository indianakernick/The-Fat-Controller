// HIToolbox/TextInputSources.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/TextInputSources.h

use super::{CFDataRef, CFStringRef};

#[repr(transparent)]
pub struct TISInputSource(std::ffi::c_void);
pub type TISInputSourceRef = *mut TISInputSource;

#[link(name = "Carbon", kind = "framework")]
extern {
    pub static kTISPropertyUnicodeKeyLayoutData: CFStringRef;

    #[allow(non_snake_case)]
    pub fn TISGetInputSourceProperty(
        inputSource: TISInputSourceRef,
        propertyKey: CFStringRef
    ) -> CFDataRef;

    pub fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
}
