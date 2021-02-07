// IOKit/IOKitLib.h

use super::*;
use std::os::raw::c_void;

#[allow(non_upper_case_globals)]
pub const kIOMasterPortDefault: mach_port_t = 0;

#[link(name = "IOKit", kind = "framework")]
extern {
    fn IOObjectRelease(object: io_object_t) -> kern_return_t;

    fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

    #[allow(non_snake_case)]
    fn IOServiceOpen(
        service: io_service_t,
        owningTask: mach_port_t,
        type_: u32,
        connect: *mut io_connect_t
    ) -> kern_return_t;

    fn IOServiceClose(connect: io_connect_t) -> kern_return_t;

    #[allow(non_snake_case)]
    fn IOServiceGetMatchingServices(
        masterPort: mach_port_t,
        matching: *const c_void,
        existing: *mut io_iterator_t
    ) -> kern_return_t;

    fn IOServiceNameMatching(name: *const u8) -> *mut c_void;
}
