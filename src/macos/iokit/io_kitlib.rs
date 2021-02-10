// IOKit/IOKitLib.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Versions/A/Headers/IOKitLib.h

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
        owningTask: task_port_t,
        type_: u32,
        connect: *mut io_connect_t
    ) -> kern_return_t;

    pub fn IOServiceClose(connect: io_connect_t) -> kern_return_t;

    #[allow(non_snake_case)]
    pub fn IOConnectMapMemory64(
        connect: io_connect_t,
        memoryType: u32,
        intoTask: task_port_t,
        atAddress: *mut mach_vm_address_t,
        ofSize: *mut mach_vm_size_t,
        options: IOOptionBits
    ) -> kern_return_t;

    #[allow(non_snake_case)]
    pub fn IOConnectUnmapMemory64(
        connect: io_connect_t,
        memoryType: u32,
        fromTask: task_port_t,
        atAddress: mach_vm_address_t
    ) -> kern_return_t;

    #[allow(non_snake_case)]
    pub fn IOServiceGetMatchingServices(
        masterPort: mach_port_t,
        matching: CGDictionaryRef,
        existing: *mut io_iterator_t
    ) -> kern_return_t;

    pub fn IOServiceMatching(name: *const u8) -> CFMutableDictionaryRef;
}
