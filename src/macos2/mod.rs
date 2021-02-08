mod key;
mod key_enum;
mod mouse;
mod mouse_button_enum;

pub use key::*;
pub use key_enum::*;
pub use mouse::*;
pub use mouse_button_enum::*;

use crate::iokit as io;

pub struct EventContext {
    service: io::io_service_t,
    connect: io::io_connect_t,
    button_state: u8,
    modifiers: io::IOOptionBits,
}

// I don't know if IOHIDPostEvent is thread-safe so I'm going to assume that it
// isn't. Also, I might need to be on nightly for the below impls to work so
// yeah...
// impl !Send for EventContext {}
// impl !Sync for EventContext {}

impl EventContext {
    pub fn new() -> Option<Self> {
        unsafe {
            // Create a dictionary that describes a service matching a name.
            let matching = io::IOServiceMatching(io::kIOHIDSystemClass.as_ptr());

            // Get an iterator to all IOService objects that match the
            // dictionary. IOServiceGetMatchingServices will release the
            // dictionary.
            let mut iterator = io::IO_OBJECT_NULL;
            if io::IOServiceGetMatchingServices(io::kIOMasterPortDefault, matching, &mut iterator) != io::kIOReturnSuccess {
                return None;
            }

            let mut found = false;
            let mut service;
            let mut connect = io::IO_OBJECT_NULL;

            // Consume the iterator and check each IOService object.
            loop {
                service = io::IOIteratorNext(iterator);
                if service == io::IO_OBJECT_NULL {
                    break;
                }

                // Try to open a connection to the IOService. If successful,
                // we're done.
                if io::IOServiceOpen(service, io::mach_task_self_, io::kIOHIDParamConnectType, &mut connect) == io::kIOReturnSuccess {
                    found = true;
                    break;
                }

                io::IOObjectRelease(service);
            }

            io::IOObjectRelease(iterator);

            if !found {
                io::IOObjectRelease(service);
                return None;
            }

            Some(EventContext {
                service,
                connect,
                button_state: 0,
                modifiers: 0,
            })
        }
    }

    fn post_event(
        &mut self,
        event_type: u32,
        event: *const io::NXEventData,
        flags: io::IOOptionBits,
        options: io::IOOptionBits
    ) -> bool {
        unsafe {
            io::IOHIDPostEvent(
                self.connect,
                event_type,
                io::IOGPoint::default(),
                event,
                io::kNXEventDataVersion,
                flags,
                options
            ) == io::kIOReturnSuccess
        }
    }
}

impl Drop for EventContext {
    fn drop(&mut self) {
        unsafe {
            io::IOServiceClose(self.connect);
            io::IOObjectRelease(self.service);
        }
    }
}
