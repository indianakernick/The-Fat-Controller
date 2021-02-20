// IOKit/hidsystem/IOHIDLib.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Versions/A/Headers/hidsystem/IOHIDLib.h

use super::*;

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
}
