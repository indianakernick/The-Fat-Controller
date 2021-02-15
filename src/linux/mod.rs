mod os;
mod error;

use std::{mem, ptr};
use std::ffi::c_void;
use std::os::raw::c_int;

pub use error::Error;

// https://www.kernel.org/doc/html/latest/input/uinput.html

pub struct Context {
    file: c_int,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let file = os::open(b"/dev/uinput".as_ptr(), os::O_WRONLY | os::O_NONBLOCK);
            if file == -1 {
                return Err(Error::errno())
            }

            let ctx = Self { file };

            ctx.ioctl(os::UI_SET_EVBIT, mem::transmute(&os::EV_KEY))?;
            ctx.ioctl(os::UI_SET_EVBIT, mem::transmute(&os::EV_REL))?;
            ctx.ioctl(os::UI_SET_EVBIT, mem::transmute(&os::EV_ABS))?;

            // TODO: only set the keys that we use
            for k in 0..256 {
                ctx.ioctl(os::UI_SET_KEYBIT, mem::transmute(&k))?;
            }

            ctx.ioctl(os::UI_SET_KEYBIT, mem::transmute(&os::BTN_LEFT))?;
            ctx.ioctl(os::UI_SET_KEYBIT, mem::transmute(&os::BTN_RIGHT))?;
            ctx.ioctl(os::UI_SET_KEYBIT, mem::transmute(&os::BTN_MIDDLE))?;

            ctx.ioctl(os::UI_SET_RELBIT, mem::transmute(&os::REL_X))?;
            ctx.ioctl(os::UI_SET_RELBIT, mem::transmute(&os::REL_Y))?;
            ctx.ioctl(os::UI_SET_RELBIT, mem::transmute(&os::REL_HWHEEL_HI_RES))?;
            ctx.ioctl(os::UI_SET_RELBIT, mem::transmute(&os::REL_WHEEL_HI_RES))?;

            ctx.ioctl(os::UI_SET_ABSBIT, mem::transmute(&os::ABS_X))?;
            ctx.ioctl(os::UI_SET_ABSBIT, mem::transmute(&os::ABS_Y))?;

            let mut setup: os::uinput_setup = mem::zeroed();
            setup.id.bustype = os::BUS_USB;
            let name = b"The Fat Controller";
            setup.name[..name.len()].copy_from_slice(name);

            ctx.ioctl(os::UI_DEV_SETUP, mem::transmute(&&setup))?;
            ctx.ioctl(os::UI_DEV_CREATE, ptr::null())?;

            Ok(ctx)
        }
    }

    fn ioctl(&self, request: u32, argp: *const c_void) -> Result<(), Error> {
        unsafe {
            if os::ioctl(self.file, request, argp) == -1 {
                return Err(Error::errno());
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            os::ioctl(self.file, os::UI_DEV_DESTROY, ptr::null());
            os::close(self.file);
        }
    }
}
