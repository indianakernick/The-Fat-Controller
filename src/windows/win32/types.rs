// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types

// WinNT.h
pub type LONG = i32;
pub type CHAR = i8;
pub type LPSTR = *mut CHAR;
pub type LPTSTR = LPSTR;

// IntSafe.h
pub type DWORD = u32;

// WinDef.h
pub type WORD = u16;
pub type LPCVOID = *const std::ffi::c_void;
pub type UINT = u32;

// BaseTsd.h
#[allow(non_camel_case_types)]
pub type ULONG_PTR = u64;
