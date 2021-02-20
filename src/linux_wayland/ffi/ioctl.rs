// https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/ioctl.h

use std::os::raw::c_int;

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;
const _IOC_SIZEBITS: u32 = 14;
const _IOC_DIRBITS: u32 = 2;

const _IOC_NRMASK: u32 = (1 << _IOC_NRBITS) - 1;
const _IOC_TYPEMASK: u32 = (1 << _IOC_TYPEBITS) - 1;
const _IOC_SIZEMASK: u32 = (1 << _IOC_SIZEBITS) - 1;
const _IOC_DIRMASK: u32 = (1 << _IOC_DIRBITS) - 1;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

#[allow(non_snake_case)]
const fn _IOC(dir: u32, type_: u32, nr: u32, size: u32) -> u32 {
    (dir << _IOC_DIRSHIFT)
        | (type_ << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | (size << _IOC_SIZESHIFT)
}

#[allow(non_snake_case)]
pub const fn _IO(type_: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, type_, nr, 0)
}

#[allow(non_snake_case)]
pub const fn _IOR<T>(type_: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, type_, nr, std::mem::size_of::<T>() as u32)
}

#[allow(non_snake_case)]
pub const fn _IOW<T>(type_: u32, nr: u32) -> u32 {
    _IOC(_IOC_WRITE, type_, nr, std::mem::size_of::<T>() as u32)
}

extern {
    // https://man7.org/linux/man-pages/man2/ioctl.2.html
    pub fn ioctl(fd: c_int, request: u32, ...) -> c_int;
}
