use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

use winapi::um::errhandlingapi::GetLastError;

// from https://drywa.me/2017/07/02/simple-win32-window-with-rust/
/// Converts a string slice to a vector which can be interpreted as an LPCWSTR.
pub fn win32_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

/// Returns the number of the last error that occured.
pub fn last_error() -> u32 {
    unsafe { GetLastError() }
}
