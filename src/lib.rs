#[derive(Debug)]
#[repr(C)]
pub struct SampleStruct {
    pub int: i32,
}

impl SampleStruct {
    #[no_mangle]
    pub extern "C" fn new() -> Self {
        SampleStruct { int: 1 }
    }
}
