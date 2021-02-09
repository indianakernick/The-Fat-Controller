// IOKit/graphics/IOGraphicsTypes.h

// Synthesised
#[allow(non_upper_case_globals)]
pub const kIOFramebufferClass: &[u8; 14] = b"IOFramebuffer\0";

#[allow(non_upper_case_globals)]
pub const kIOFBServerConnectType: u32 = 0;
#[allow(non_upper_case_globals)]
pub const kIOFBSharedConnectType: u32 = 1;
#[allow(non_upper_case_globals)]
pub const kIOGDiagnoseConnectType: u32 = 38744;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOGPoint {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOGBounds {
    pub minx: i16,
    pub maxx: i16,
    pub miny: i16,
    pub maxy: i16,
}
