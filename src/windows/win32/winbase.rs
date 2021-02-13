// Winbase.h

use super::types::*;

#[link(name = "kernel32")]
extern "stdcall" {
    // FormatMessageA - ANSI
    // FormatMessageW - Unicode

    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage
    #[allow(non_snake_case)]
    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        Arguments: *mut std::ffi::c_void, // va_list
    ) -> DWORD;

    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree
    #[allow(non_snake_case)]
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;
}

pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x0100;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x2000;
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x0800;
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x0400;
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x1000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x0200;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: DWORD = 0x00FF;
