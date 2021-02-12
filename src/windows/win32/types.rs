// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types

use std::ffi::c_void;

// WinNT.h
pub type LONG = i32;
pub type LPTSTR = *mut u8;
pub type LPCTSTR = *const u8;
pub type PVOID = *mut c_void;
pub type HANDLE = PVOID;

// IntSafe.h
pub type DWORD = u32;
pub type NonZeroDWORD = std::num::NonZeroU32;

// WinDef.h
pub type WORD = u16;
pub type LPCVOID = *const c_void;
pub type LPVOID = *mut c_void;
pub type UINT = u32;
pub type HLOCAL = HANDLE;

// BaseTsd.h
#[allow(non_camel_case_types)]
pub type ULONG_PTR = u64;
