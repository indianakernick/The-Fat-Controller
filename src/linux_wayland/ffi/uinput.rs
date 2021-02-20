// https://github.com/torvalds/linux/blob/master/include/uapi/linux/uinput.h

use std::os::raw::c_int;
use super::{input_id, _IO, _IOW};

const UINPUT_MAX_NAME_SIZE: usize = 80;

const UINPUT_IOCTL_BASE: u32 = 'U' as u32;

pub const UI_DEV_CREATE: u32 = _IO(UINPUT_IOCTL_BASE, 1);
pub const UI_DEV_DESTROY: u32 = _IO(UINPUT_IOCTL_BASE, 2);

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct uinput_setup {
    pub id: input_id,
    pub name: [u8; UINPUT_MAX_NAME_SIZE],
    pub ff_effects_max: u32,
}

pub const UI_DEV_SETUP: u32 = _IOW::<uinput_setup>(UINPUT_IOCTL_BASE, 3);
pub const UI_SET_EVBIT: u32 = _IOW::<c_int>(UINPUT_IOCTL_BASE, 100);
pub const UI_SET_KEYBIT: u32 = _IOW::<c_int>(UINPUT_IOCTL_BASE, 101);
pub const UI_SET_RELBIT: u32 = _IOW::<c_int>(UINPUT_IOCTL_BASE, 102);
pub const UI_SET_ABSBIT: u32 = _IOW::<c_int>(UINPUT_IOCTL_BASE, 103);
