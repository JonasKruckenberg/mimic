use mimic::generate_mimic;

#[test]
fn basic() {
    pub struct Foo {
        a: u64,
    }
    generate_mimic!(FooMimic for Foo);

    let ty = Foo { a: 42 };
    let mimic = FooMimic::new();

    assert_eq!(size_of_val(&ty), size_of_val(&mimic));
    assert_eq!(align_of_val(&ty), align_of_val(&mimic));
}

#[test]
fn derive_and_attrs() {
    pub struct Foo {
        a: u64,
    }
    generate_mimic! {
        #[derive(Debug)]
        #[doc = "foo"]
        #[repr(C)]
        FooMimic for Foo
    }

    let ty = Foo { a: 42 };
    let mimic = FooMimic::new();

    println!("{mimic:?}");

    assert_eq!(size_of_val(&ty), size_of_val(&mimic));
    assert_eq!(align_of_val(&ty), align_of_val(&mimic));
}

#[test]
fn generics() {
    pub struct Foo<T> {
        a: u64,
        b: T,
    }
    generate_mimic!(FooMimicU64 for Foo<u64>);

    let ty_u64 = Foo { a: 42, b: 120_u64 };
    let mimic_u64 = FooMimicU64::new();

    assert_eq!(size_of_val(&ty_u64), size_of_val(&mimic_u64));
    assert_eq!(align_of_val(&ty_u64), align_of_val(&mimic_u64));

    // this still works even though `FooMimicString` doesn't know the exact size of the "hello_world"
    // since that is stored on the heap anyway
    generate_mimic!(FooMimicString for Foo<String>);
    let ty_str = Foo {
        a: 42,
        b: "hello_world".to_string(),
    };
    let mimic_str = FooMimicString::new();

    assert_eq!(size_of_val(&ty_str), size_of_val(&mimic_str));
    assert_eq!(align_of_val(&ty_str), align_of_val(&mimic_str));
}
