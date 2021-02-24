use super::{ffi, Error};

impl crate::ScreenContext for super::Context {
    fn cursor_location(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let struct_ptr = self.fb_address as *const ffi::StdFBShmem_t;
            let loc_ptr: *const ffi::IOGPoint = &(*struct_ptr).cursorLoc;
            let loc = std::ptr::read_volatile(loc_ptr);
            Ok((loc.x as i32, loc.y as i32))
        }
    }

    fn screen_size(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let struct_ptr = self.fb_address as *const ffi::StdFBShmem_t;
            let bounds_ptr: *const ffi::IOGBounds = &(*struct_ptr).screenBounds;
            let bounds = std::ptr::read_volatile(bounds_ptr);
            Ok(((bounds.maxx - bounds.minx) as i32, (bounds.maxy - bounds.miny) as i32))
        }
    }
}
