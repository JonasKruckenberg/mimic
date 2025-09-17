# `mimic`

Mimic generates "type-erased" struct definitions of *equal size* and *equal alignment*.
This is useful in FFI scenarios where you don't want to or cannot communicate the exact layout and type of fields to the other language,
but still want to communicate it's size and alignment e.g. to be able to stack-allocate the type.

```rust
fn main() {
    pub struct Foo {
        a: u64,
    }
    generate_mimic!(FooMimic for Foo);

    let ty = Foo { a: 42 };
    let mimic = FooMimic::new();

    assert_eq!(size_of_val(&ty), size_of_val(&mimic));
    assert_eq!(align_of_val(&ty), align_of_val(&mimic));
}
```
