// IOKit/IOTypes.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Versions/A/Headers/IOTypes.h

pub type IOOptionBits = u32;

#[allow(non_camel_case_types)]
pub type io_object_t = super::mach_port_t;
#[allow(non_camel_case_types)]
pub type io_connect_t = io_object_t;
#[allow(non_camel_case_types)]
pub type io_iterator_t = io_object_t;
#[allow(non_camel_case_types)]
pub type io_service_t = io_object_t;

pub const IO_OBJECT_NULL: io_object_t = 0;
