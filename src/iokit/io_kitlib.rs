// IOKit/IOKitLib.h

use super::*;
use std::os::raw::c_void;

#[allow(non_upper_case_globals)]
pub const kIOMasterPortDefault: mach_port_t = 0;

pub type CGDictionaryRef = *const c_void;
pub type CFMutableDictionaryRef = *mut c_void;

#[link(name = "IOKit", kind = "framework")]
extern {
    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;

    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

    #[allow(non_snake_case)]
    pub fn IOServiceOpen(
        service: io_service_t,
        owningTask: mach_port_t,
        type_: u32,
        connect: *mut io_connect_t
    ) -> kern_return_t;

    pub fn IOServiceClose(connect: io_connect_t) -> kern_return_t;

    #[allow(non_snake_case)]
    pub fn IOServiceGetMatchingServices(
        masterPort: mach_port_t,
        matching: CGDictionaryRef,
        existing: *mut io_iterator_t
    ) -> kern_return_t;

    pub fn IOServiceNameMatching(name: *const u8) -> CFMutableDictionaryRef;
}
