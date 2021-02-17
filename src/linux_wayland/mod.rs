mod os;
mod error;
mod key;
mod mouse;

use std::mem;
use crate::Key;
use std::os::raw::c_int;
use crate::linux_common::{self, ScrollAccum};

pub use error::Error;

// https://www.kernel.org/doc/html/latest/input/uinput.html

/// The main context used for generating events (Linux-Wayland).
///
/// The most useful methods are on the
/// [`KeyboardContext`](crate::KeyboardContext) and
/// [`MouseContext`](crate::MouseContext) traits.
///
/// The Linux-Wayland context doesn't implement
/// [`InfoContext`](crate::InfoContext) but the Linux-X11 context does.
pub struct Context {
    file: c_int,
    scroll: ScrollAccum,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let file = os::open(b"/dev/uinput\0".as_ptr(), os::O_WRONLY | os::O_NONBLOCK);
            if file == -1 {
                return Err(Error::errno())
            }

            let ctx = Self { file, scroll: ScrollAccum::default() };

            ctx.ioctl(os::UI_SET_EVBIT, os::EV_KEY)?;
            ctx.ioctl(os::UI_SET_EVBIT, os::EV_REL)?;

            for k in 0..Key::COUNT {
                let key = std::mem::transmute(k);
                let key_code = linux_common::to_key_code(key) as c_int;
                ctx.ioctl(os::UI_SET_KEYBIT, key_code)?;
            }

            ctx.ioctl(os::UI_SET_KEYBIT, os::BTN_LEFT)?;
            ctx.ioctl(os::UI_SET_KEYBIT, os::BTN_RIGHT)?;
            ctx.ioctl(os::UI_SET_KEYBIT, os::BTN_MIDDLE)?;

            ctx.ioctl(os::UI_SET_RELBIT, os::REL_X)?;
            ctx.ioctl(os::UI_SET_RELBIT, os::REL_Y)?;
            // ctx.ioctl(os::UI_SET_RELBIT, os::REL_HWHEEL_HI_RES)?;
            // ctx.ioctl(os::UI_SET_RELBIT, os::REL_WHEEL_HI_RES)?;
            ctx.ioctl(os::UI_SET_RELBIT, os::REL_HWHEEL)?;
            ctx.ioctl(os::UI_SET_RELBIT, os::REL_WHEEL)?;

            let mut setup: os::uinput_setup = mem::zeroed();
            setup.id.bustype = os::BUS_USB;
            let name = b"The Fat Controller";
            setup.name[..name.len()].copy_from_slice(name);

            ctx.ioctl(os::UI_DEV_SETUP, &setup)?;
            ctx.ioctl_0(os::UI_DEV_CREATE)?;

            Ok(ctx)
        }
    }

    fn ioctl<T>(&self, request: u32, arg: T) -> Result<(), Error> {
        unsafe {
            if os::ioctl(self.file, request, arg) == -1 {
                Err(Error::errno())
            } else {
                Ok(())
            }
        }
    }

    fn ioctl_0(&self, request: u32) -> Result<(), Error> {
        unsafe {
            if os::ioctl(self.file, request) == -1 {
                Err(Error::errno())
            } else {
                Ok(())
            }
        }
    }

    fn write(&self, type_: u16, code: u16, value: i32) -> Result<(), Error> {
        unsafe {
            let event = os::input_event {
                time: os::timeval {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                type_,
                code,
                value,
            };
            let size = mem::size_of::<os::input_event>();
            let written = os::write(self.file, mem::transmute(&event), size);
            if written == -1 {
                Err(Error::errno())
            } else if written != size as isize {
                Err(Error::unknown())
            } else {
                Ok(())
            }
        }
    }

    fn write_syn_report(&self) -> Result<(), Error> {
        self.write(os::EV_SYN, os::SYN_REPORT, 0)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            os::ioctl(self.file, os::UI_DEV_DESTROY);
            os::close(self.file);
        }
    }
}
