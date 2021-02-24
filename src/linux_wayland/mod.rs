mod ffi;
mod error;
mod keyboard;
mod mouse;

// The implementation of this module is adapted from here:
// https://www.kernel.org/doc/html/latest/input/uinput.html

use error::PlatformError;
type Error = crate::GenericError<PlatformError>;

/// The main context used for generating events (Linux-Wayland).
///
/// The most useful methods are on the [`traits`](crate::traits) however the
/// Linux-Wayland context doesn't implement [`InfoContext`](crate::InfoContext)
/// or [`UnicodeKeyboardContext`](crate::UnicodeKeyboardContext) but the
/// Linux-X11 context does.
pub struct Context {
    file: std::os::raw::c_int,
    scroll: crate::linux_common::ScrollAccum,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let file = unsafe {
            ffi::open(b"/dev/uinput\0".as_ptr(), ffi::O_WRONLY | ffi::O_NONBLOCK)
        };
        if file == -1 {
            return Err(Error::Platform(PlatformError::errno()))
        }

        let ctx = Self { file, scroll: Default::default() };

        ctx.ioctl(ffi::UI_SET_EVBIT, ffi::EV_KEY)?;
        ctx.ioctl(ffi::UI_SET_EVBIT, ffi::EV_REL)?;

        for k in 0..crate::Key::COUNT {
            let key = unsafe { std::mem::transmute(k) };
            let key_code = crate::linux_common::to_key_code(key) as std::os::raw::c_int;
            ctx.ioctl(ffi::UI_SET_KEYBIT, key_code)?;
        }

        ctx.ioctl(ffi::UI_SET_KEYBIT, ffi::BTN_LEFT)?;
        ctx.ioctl(ffi::UI_SET_KEYBIT, ffi::BTN_RIGHT)?;
        ctx.ioctl(ffi::UI_SET_KEYBIT, ffi::BTN_MIDDLE)?;

        ctx.ioctl(ffi::UI_SET_RELBIT, ffi::REL_X)?;
        ctx.ioctl(ffi::UI_SET_RELBIT, ffi::REL_Y)?;
        // ctx.ioctl(ffi::UI_SET_RELBIT, ffi::REL_HWHEEL_HI_RES)?;
        // ctx.ioctl(ffi::UI_SET_RELBIT, ffi::REL_WHEEL_HI_RES)?;
        ctx.ioctl(ffi::UI_SET_RELBIT, ffi::REL_HWHEEL)?;
        ctx.ioctl(ffi::UI_SET_RELBIT, ffi::REL_WHEEL)?;

        let mut setup: ffi::uinput_setup = unsafe { std::mem::zeroed() };
        setup.id.bustype = ffi::BUS_USB;
        let name = b"The Fat Controller";
        setup.name[..name.len()].copy_from_slice(name);

        ctx.ioctl(ffi::UI_DEV_SETUP, &setup)?;
        ctx.ioctl_0(ffi::UI_DEV_CREATE)?;

        Ok(ctx)
    }

    fn ioctl<T>(&self, request: u32, arg: T) -> Result<(), Error> {
        unsafe {
            if ffi::ioctl(self.file, request, arg) == -1 {
                Err(Error::Platform(PlatformError::errno()))
            } else {
                Ok(())
            }
        }
    }

    fn ioctl_0(&self, request: u32) -> Result<(), Error> {
        unsafe {
            if ffi::ioctl(self.file, request) == -1 {
                Err(Error::Platform(PlatformError::errno()))
            } else {
                Ok(())
            }
        }
    }

    fn write(&self, type_: u16, code: u16, value: i32) -> Result<(), Error> {
        let event = ffi::input_event {
            time: ffi::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            type_,
            code,
            value,
        };
        let size = std::mem::size_of::<ffi::input_event>();
        let written = unsafe {
            ffi::write(self.file, std::mem::transmute(&event), size)
        };
        if written == -1 {
            Err(Error::Platform(PlatformError::errno()))
        } else if written != size as isize {
            Err(Error::Unknown)
        } else {
            Ok(())
        }
    }

    fn write_syn_report(&self) -> Result<(), Error> {
        self.write(ffi::EV_SYN, ffi::SYN_REPORT, 0)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::ioctl(self.file, ffi::UI_DEV_DESTROY);
            ffi::close(self.file);
        }
    }
}
