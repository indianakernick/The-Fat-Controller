// IOKit/hidsystem/IOHIDLib.h

use super::*;
use std::os::raw::c_int;

#[allow(non_upper_case_globals)]
pub const kIOHIDSetGlobalEventFlags: IOOptionBits = 1;
#[allow(non_upper_case_globals)]
pub const kIOHIDSetCursorPosition: IOOptionBits = 2;
#[allow(non_upper_case_globals)]
pub const kIOHIDSetRelativeCursorPosition: IOOptionBits = 4;
#[allow(non_upper_case_globals)]
pub const kIOHIDPostHIDManagerEvent: IOOptionBits = 8;

#[link(name = "IOKit", kind = "framework")]
extern {
    #[allow(non_snake_case)]
    pub fn IOHIDPostEvent(
        connect: io_connect_t,
        eventType: u32,
        location: IOGPoint,
        eventData: *const NXEventData,
        eventDataVersion: u32,
        eventFlags: IOOptionBits,
        options: IOOptionBits
    ) -> kern_return_t;

    pub fn IOHIDSetMouseLocation(connect: io_connect_t, x: c_int, y: c_int) -> kern_return_t;
}
