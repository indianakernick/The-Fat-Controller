mod ffi;
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
    hid_connect: ffi::io_connect_t,
    fb_connect: ffi::io_connect_t,
    fb_address: ffi::mach_vm_address_t,
    modifiers: ffi::IOOptionBits,
    button_state: u8,
}

// I don't know if IOHIDPostEvent is thread-safe so I'm going to assume that it
// isn't. Also, I might need to be on nightly for the below impls to work so
// yeah...
// impl !Send for EventContext {}
// impl !Sync for EventContext {}

fn connect_to_service(name: *const u8, connect_type: u32) -> Result<ffi::io_connect_t, Error> {
    unsafe {
        // Create a dictionary that describes a service matching a name.
        let matching = ffi::IOServiceMatching(name);

        // Get an iterator to all IOService objects that match the
        // dictionary. IOServiceGetMatchingServices will release the
        // dictionary.
        let mut iterator = ffi::IO_OBJECT_NULL;
        let error_code = ffi::IOServiceGetMatchingServices(ffi::kIOMasterPortDefault, matching, &mut iterator);
        if error_code != ffi::kIOReturnSuccess {
            return Err(Error::new(error_code));
        }

        let mut found = false;
        let mut service;
        let mut connect = ffi::IO_OBJECT_NULL;

        // Consume the iterator and check each IOService object.
        loop {
            service = ffi::IOIteratorNext(iterator);
            if service == ffi::IO_OBJECT_NULL {
                break;
            }

            // Try to open a connection to the IOService. If successful,
            // we're done. We don't need a reference to the service after
            // opening a connection to it.
            if ffi::IOServiceOpen(service, ffi::mach_task_self_, connect_type, &mut connect) == ffi::kIOReturnSuccess {
                found = true;
                ffi::IOObjectRelease(service);
                break;
            }

            ffi::IOObjectRelease(service);
        }

        ffi::IOObjectRelease(iterator);

        if found {
            Ok(connect)
        } else {
            Err(Error::new(ffi::kIOReturnError))
        }
    }
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let hid_connect = connect_to_service(ffi::kIOHIDSystemClass.as_ptr(), ffi::kIOHIDParamConnectType)?;

        let fb_connect = match connect_to_service(ffi::kIOFramebufferClass.as_ptr(), ffi::kIOFBSharedConnectType) {
            Ok(connect) => connect,
            Err(e) => {
                unsafe {
                    ffi::IOServiceClose(hid_connect);
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
            let error_code = ffi::IOConnectMapMemory64(
                fb_connect,
                ffi::kIOFBCursorMemory,
                ffi::mach_task_self_,
                &mut fb_address,
                &mut size,
                ffi::kIOMapAnywhere
            );
            if error_code != ffi::kIOReturnSuccess {
                ffi::IOServiceClose(fb_connect);
                ffi::IOServiceClose(hid_connect);
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
        event: *const ffi::NXEventData,
        flags: ffi::IOOptionBits,
        options: ffi::IOOptionBits
    ) -> Result<(), Error> {
        let error_code;
        unsafe {
            error_code = ffi::IOHIDPostEvent(
                self.hid_connect,
                event_type,
                ffi::IOGPoint{ x: 0, y: 0 },
                event,
                ffi::kNXEventDataVersion,
                flags,
                options
            );
        }
        if error_code == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(Error::new(error_code))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::IOConnectUnmapMemory64(
                self.fb_connect,
                ffi::kIOFBCursorMemory,
                ffi::mach_task_self_,
                self.fb_address
            );
            ffi::IOServiceClose(self.fb_connect);
            ffi::IOServiceClose(self.hid_connect);
        }
    }
}
