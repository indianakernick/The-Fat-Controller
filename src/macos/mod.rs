mod os;
mod error;
mod info;
mod keyboard;
mod mouse;

pub use error::Error;

/// The main context used for generating events (macOS).
///
/// The most useful methods are on the [`InfoContext`](crate::InfoContext),
/// [`KeyboardContext`](crate::KeyboardContext) and
/// [`MouseContext`](crate::MouseContext) traits.
pub struct Context {
    hid_connect: os::io_connect_t,
    fb_connect: os::io_connect_t,
    fb_address: os::mach_vm_address_t,
    modifiers: os::IOOptionBits,
    button_state: u8,
}

// I don't know if IOHIDPostEvent is thread-safe so I'm going to assume that it
// isn't. Also, I might need to be on nightly for the below impls to work so
// yeah...
// impl !Send for EventContext {}
// impl !Sync for EventContext {}

fn connect_to_service(name: *const u8, connect_type: u32) -> Result<os::io_connect_t, Error> {
    unsafe {
        // Create a dictionary that describes a service matching a name.
        let matching = os::IOServiceMatching(name);

        // Get an iterator to all IOService objects that match the
        // dictionary. IOServiceGetMatchingServices will release the
        // dictionary.
        let mut iterator = os::IO_OBJECT_NULL;
        let error_code = os::IOServiceGetMatchingServices(os::kIOMasterPortDefault, matching, &mut iterator);
        if error_code != os::kIOReturnSuccess {
            return Err(Error::new(error_code));
        }

        let mut found = false;
        let mut service;
        let mut connect = os::IO_OBJECT_NULL;

        // Consume the iterator and check each IOService object.
        loop {
            service = os::IOIteratorNext(iterator);
            if service == os::IO_OBJECT_NULL {
                break;
            }

            // Try to open a connection to the IOService. If successful,
            // we're done. We don't need a reference to the service after
            // opening a connection to it.
            if os::IOServiceOpen(service, os::mach_task_self_, connect_type, &mut connect) == os::kIOReturnSuccess {
                found = true;
                os::IOObjectRelease(service);
                break;
            }

            os::IOObjectRelease(service);
        }

        os::IOObjectRelease(iterator);

        if found {
            Ok(connect)
        } else {
            Err(Error::new(os::kIOReturnError))
        }
    }
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let hid_connect = connect_to_service(os::kIOHIDSystemClass.as_ptr(), os::kIOHIDParamConnectType)?;

        let fb_connect = match connect_to_service(os::kIOFramebufferClass.as_ptr(), os::kIOFBSharedConnectType) {
            Ok(connect) => connect,
            Err(e) => {
                unsafe {
                    os::IOServiceClose(hid_connect);
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
            let error_code = os::IOConnectMapMemory64(
                fb_connect,
                os::kIOFBCursorMemory,
                os::mach_task_self_,
                &mut fb_address,
                &mut size,
                os::kIOMapAnywhere
            );
            if error_code != os::kIOReturnSuccess {
                os::IOServiceClose(fb_connect);
                os::IOServiceClose(hid_connect);
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
        event: *const os::NXEventData,
        flags: os::IOOptionBits,
        options: os::IOOptionBits
    ) -> Result<(), Error> {
        let error_code;
        unsafe {
            error_code = os::IOHIDPostEvent(
                self.hid_connect,
                event_type,
                os::IOGPoint{ x: 0, y: 0 },
                event,
                os::kNXEventDataVersion,
                flags,
                options
            );
        }
        if error_code == os::kIOReturnSuccess {
            Ok(())
        } else {
            Err(Error::new(error_code))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            os::IOConnectUnmapMemory64(
                self.fb_connect,
                os::kIOFBCursorMemory,
                os::mach_task_self_,
                self.fb_address
            );
            os::IOServiceClose(self.fb_connect);
            os::IOServiceClose(self.hid_connect);
        }
    }
}
