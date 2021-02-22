// CoreFoundation/CFBase.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/CFBase.h

use std::ffi::c_void;

#[repr(transparent)]
pub struct CFString(c_void);
pub type CFStringRef = *const CFString;

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    pub fn CFRelease(cf: *mut c_void);
}
