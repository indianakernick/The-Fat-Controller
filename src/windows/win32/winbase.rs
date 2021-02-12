// std::ffi::VaList is nightly-only.

/*
#![feature(c_variadic)]

use super::types::*;

extern "C" {
    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage
    #[allow(non_snake_case)]
    pub fn FormatMessage(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPTSTR,
        nSize: DWORD,
        Arguments: *mut std::ffi::VaList,
    ) -> DWORD;
}
*/