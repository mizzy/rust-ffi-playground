extern crate ffi_playground;

use ffi_playground::SampleStruct;

fn main() {
    let s = SampleStruct::new();
    println!("{}", s.int);
}
