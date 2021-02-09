mod iokit;
mod key;
mod mouse;

pub use key::*;
pub use mouse::*;

use iokit as io;

pub struct Context {
    hid_connect: io::io_connect_t,
    fb_connect: io::io_connect_t,
    fb_address: io::mach_vm_address_t,
    modifiers: io::IOOptionBits,
    button_state: u8,
}

// I don't know if IOHIDPostEvent is thread-safe so I'm going to assume that it
// isn't. Also, I might need to be on nightly for the below impls to work so
// yeah...
// impl !Send for EventContext {}
// impl !Sync for EventContext {}

fn connect_to_service(name: *const u8, connect_type: u32) -> Option<io::io_connect_t> {
    unsafe {
        // Create a dictionary that describes a service matching a name.
        let matching = io::IOServiceMatching(name);

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
            // we're done. We don't need a reference to the service after
            // opening a connection to it.
            if io::IOServiceOpen(service, io::mach_task_self_, connect_type, &mut connect) == io::kIOReturnSuccess {
                found = true;
                io::IOObjectRelease(service);
                break;
            }

            io::IOObjectRelease(service);
        }

        io::IOObjectRelease(iterator);

        if found {
            Some(connect)
        } else {
            None
        }
    }
}

impl Context {
    pub fn new() -> Option<Self> {
        let hid_connect = match connect_to_service(io::kIOHIDSystemClass.as_ptr(), io::kIOHIDParamConnectType) {
            Some(connect) => connect,
            None => return None,
        };

        let fb_connect = match connect_to_service(io::kIOFramebufferClass.as_ptr(), io::kIOFBSharedConnectType) {
            Some(connect) => connect,
            None => {
                unsafe {
                    io::IOServiceClose(hid_connect);
                }
                return None;
            },
        };

        let mut fb_address = 0;
        unsafe {
            let mut size = 0;
            if io::IOConnectMapMemory64(
                fb_connect,
                io::kIOFBCursorMemory,
                io::mach_task_self_,
                &mut fb_address,
                &mut size,
                io::kIOMapAnywhere
            ) != io::kIOReturnSuccess {
                io::IOServiceClose(fb_connect);
                io::IOServiceClose(hid_connect);
                return None;
            }
        }

        Some(Context {
            hid_connect,
            fb_connect,
            fb_address,
            modifiers: 0,
            button_state: 0,
        })
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
                self.hid_connect,
                event_type,
                io::IOGPoint{ x: 0, y: 0 },
                event,
                io::kNXEventDataVersion,
                flags,
                options
            ) == io::kIOReturnSuccess
        }
    }

    pub fn mouse_location(&mut self) -> (i32, i32) {
        unsafe {
            let struct_ptr = self.fb_address as *const io::StdFBShmem_t;
            let loc_ptr: *const io::IOGPoint = &(*struct_ptr).cursorLoc;
            let loc = std::ptr::read_volatile(loc_ptr);
            (loc.x as i32, loc.y as i32)
        }
    }

    pub fn screen_size(&mut self) -> (i32, i32) {
        unsafe {
            let struct_ptr = self.fb_address as *const io::StdFBShmem_t;
            let bounds_ptr: *const io::IOGBounds = &(*struct_ptr).screenBounds;
            let bounds = std::ptr::read_volatile(bounds_ptr);
            ((bounds.maxx - bounds.minx) as i32, (bounds.maxy - bounds.miny) as i32)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            io::IOConnectUnmapMemory64(
                self.fb_connect,
                io::kIOFBCursorMemory,
                io::mach_task_self_,
                self.fb_address
            );
            io::IOServiceClose(self.fb_connect);
            io::IOServiceClose(self.hid_connect);
        }
    }
}
