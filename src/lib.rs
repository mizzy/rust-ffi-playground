use std::fmt::Debug;

pub trait FooTrait: Debug {
    fn new() -> Self where Self: Sized;
}

#[derive(Debug)]
pub struct Foo;

impl FooTrait for Foo {
    fn new() -> Foo {
        Foo
    }
}

#[no_mangle]
pub extern "C" fn foo_free(ptr: *mut Foo) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn foo_new() -> *const Foo {
    Box::into_raw(Box::new(Foo::new()))
}

#[no_mangle]
pub extern "C" fn foo_take_as_struct(ptr: *const Foo) {
    let foo = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    println!("{:?}", foo);
}

#[no_mangle]
pub extern "C" fn foo_take_as_trait(ptr: *const FooTrait) {
    let foo = unsafe {
        // assert!(!ptr.is_null());
        //              ^^^^^^^ the trait `std::marker::Sized` is not implemented for `FooTrait`
        &*ptr
    };

    println!("{:?}", foo);
}
