// CoreFoundation/CFData.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/CFData.h

#[repr(transparent)]
pub struct CFData(std::ffi::c_void);
pub type CFDataRef = *const CFData;

#[link(name = "CoreFoundation", kind = "framework")]
extern {
    #[allow(non_snake_case)]
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
}
