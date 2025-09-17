use mimic::{Size8Align8, Size40Align8, impl_mimic};
use std::hint::black_box;

#[test]
fn basic() {
    #[allow(unused)]
    pub struct Foo {
        a: u64,
    }
    pub struct FooMimic(Size8Align8);
    impl_mimic!(FooMimic for Foo);

    let ty = Foo { a: 42 };
    let mimic = FooMimic(unsafe { Size8Align8::new_uninit() });

    assert_eq!(size_of_val(&ty), size_of_val(&mimic));
    assert_eq!(align_of_val(&ty), align_of_val(&mimic));
}

#[test]
fn conversion() {
    #[derive(Debug, PartialEq)]
    pub struct Foo {
        a: u64,
    }
    pub struct FooMimic(Size8Align8);
    impl_mimic!(FooMimic for Foo);

    let a = Foo { a: 42 };
    let b = Foo { a: 42 };

    let b = unsafe { black_box(FooMimic::from_ty(b)).into_ty() };

    assert_eq!(a, b);
    assert_eq!(a, b);
}

#[test]
fn with_lifetime() {
    #[derive(Debug, PartialEq)]
    pub struct Foo<'a> {
        a: &'a u64,
    }
    pub struct FooMimic(Size8Align8);
    impl_mimic!(FooMimic for Foo<'static>);

    static A_VAL: u64 = 42;
    static B_VAL: u64 = 42;

    let a = Foo { a: &A_VAL };
    let b = Foo { a: &B_VAL };

    let b = unsafe { black_box(FooMimic::from_ty(b)).into_ty() };

    assert_eq!(a, b);
    assert_eq!(a, b);
}

#[test]
fn with_generics() {
    #[derive(Debug, PartialEq)]
    pub struct Foo<T, U> {
        a: u64,
        b: T,
        c: U,
    }
    pub struct FooMimic(Size40Align8);
    impl_mimic!(FooMimic for Foo<u8, String>);

    let a = Foo {
        a: 42,
        b: 8,
        c: "hello_world".to_string(),
    };
    let b = Foo {
        a: 42,
        b: 8,
        c: "hello_world".to_string(),
    };

    let b = unsafe { black_box(FooMimic::from_ty(b)).into_ty() };

    assert_eq!(a, b);
    assert_eq!(a, b);
}
