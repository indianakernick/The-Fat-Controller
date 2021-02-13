mod iokit;
mod error;
mod info;
mod key;
mod mouse;

use iokit as io;

pub use error::Error;

/// The main context used for generating events.
///
/// The most useful methods are on the [`InfoContext`](crate::InfoContext),
/// [`KeyboardContext`](crate::KeyboardContext) and
/// [`MouseContext`](crate::MouseContext) traits.
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

fn connect_to_service(name: *const u8, connect_type: u32) -> Result<io::io_connect_t, Error> {
    unsafe {
        // Create a dictionary that describes a service matching a name.
        let matching = io::IOServiceMatching(name);

        // Get an iterator to all IOService objects that match the
        // dictionary. IOServiceGetMatchingServices will release the
        // dictionary.
        let mut iterator = io::IO_OBJECT_NULL;
        let error_code = io::IOServiceGetMatchingServices(io::kIOMasterPortDefault, matching, &mut iterator);
        if error_code != io::kIOReturnSuccess {
            return Err(Error::new(error_code));
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
            Ok(connect)
        } else {
            Err(Error::new(io::kIOReturnError))
        }
    }
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let hid_connect = connect_to_service(io::kIOHIDSystemClass.as_ptr(), io::kIOHIDParamConnectType)?;

        let fb_connect = match connect_to_service(io::kIOFramebufferClass.as_ptr(), io::kIOFBSharedConnectType) {
            Ok(connect) => connect,
            Err(e) => {
                unsafe {
                    io::IOServiceClose(hid_connect);
                }
                return Err(e);
            },
        };

        // Memory mapping IOFramebuffer to get StdFBShmem_t won't work on Apple
        // Silicon. Instead, the properties of IOMobileFramebuffer need to be
        // inspected.
        // Maybe we should just use Core Graphics

        let mut fb_address = 0;
        unsafe {
            let mut size = 0;
            let error_code = io::IOConnectMapMemory64(
                fb_connect,
                io::kIOFBCursorMemory,
                io::mach_task_self_,
                &mut fb_address,
                &mut size,
                io::kIOMapAnywhere
            );
            if error_code != io::kIOReturnSuccess {
                io::IOServiceClose(fb_connect);
                io::IOServiceClose(hid_connect);
                return Err(Error::new(error_code));
            }
        }

        Ok(Self {
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
    ) -> Result<(), Error> {
        let error_code;
        unsafe {
            error_code = io::IOHIDPostEvent(
                self.hid_connect,
                event_type,
                io::IOGPoint{ x: 0, y: 0 },
                event,
                io::kNXEventDataVersion,
                flags,
                options
            );
        }
        if error_code == io::kIOReturnSuccess {
            Ok(())
        } else {
            Err(Error::new(error_code))
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
