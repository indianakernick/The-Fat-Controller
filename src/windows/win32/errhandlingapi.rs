// errhandlingapi.h

use super::types::*;

extern "C" {
    // https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror
    pub fn GetLastError() -> DWORD;
}
