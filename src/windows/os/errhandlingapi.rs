// errhandlingapi.h

use super::types::*;

#[link(name = "kernel32")]
extern "stdcall" {
    // https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
    pub fn GetLastError() -> DWORD;
}
