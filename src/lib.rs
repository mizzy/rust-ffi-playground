extern crate libc;

use libc::*;
use std::ffi::CString;

#[derive(Debug)]
#[repr(C)]
pub struct SampleStruct {
    pub int: i32,
    pub str: *const c_char,
}

impl SampleStruct {
    #[no_mangle]
    pub extern "C" fn new() -> Self {
        SampleStruct {
            int: 1,
            str: CString::new("aaaaaa").unwrap().into_raw(),
        }
    }
}
