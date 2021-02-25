// IOKit/graphics/IOGraphicsTypes.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Versions/A/Headers/graphics/IOGraphicsTypes.h

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOGPoint {
    pub x: i16,
    pub y: i16,
}
