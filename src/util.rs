use std::ffi::CString;

extern crate libc;

pub fn to_c_str(s: &str) -> *const libc::c_char {
    CString::new(s).unwrap().as_ptr()
}
