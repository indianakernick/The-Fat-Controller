#![allow(clippy::upper_case_acronyms)]

// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types

use std::ffi::c_void;

// WinNT.h
pub type SHORT = i16;
pub type LONG = i32;
pub type WCHAR = u16;
pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type PVOID = *mut c_void;
pub type HANDLE = PVOID;

// IntSafe.h
pub type DWORD = u32;

// WinDef.h
pub type WORD = u16;
pub type LPCVOID = *const c_void;
pub type UINT = u32;
pub type HLOCAL = HANDLE;
pub type BOOL = i32;

// BaseTsd.h
#[allow(non_camel_case_types)]
pub type ULONG_PTR = u64;

// Not sure where this is defined

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

pub type LPPOINT = *mut POINT;
