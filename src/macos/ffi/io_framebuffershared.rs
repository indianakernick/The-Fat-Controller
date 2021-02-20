// IOKit/graphics/IOFramebufferShared.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Versions/A/Headers/graphics/IOFramebufferShared.h

use super::{IOGPoint, IOGBounds};

// StdFBShmem_t is actually a bit bigger than this but I don't need all of it.
// Plus some parts of it are conditionally compiled in.

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct StdFBShmem_t {
    pub cursorSema: i32,
    pub frame: std::os::raw::c_int,
    pub cursorShow: i8,
    pub cursorObscured: i8,
    pub shieldFlag: i8,
    pub shielded: i8,
    pub saveRect: IOGBounds,
    pub shieldRect: IOGBounds,
    pub cursorLoc: IOGPoint,
    pub cursorRect: IOGBounds,
    pub oldCursorRect: IOGBounds,
    pub screenBounds: IOGBounds,
}

#[allow(non_upper_case_globals)]
pub const kIOFBCursorMemory: u32 = 100;
