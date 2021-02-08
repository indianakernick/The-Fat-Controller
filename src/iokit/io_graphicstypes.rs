// IOKit/graphics/IOGraphicsTypes.h

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IOGPoint {
    pub x: i16,
    pub y: i16,
}
