use std::{
    fmt::Write,
    fs::{self},
};

const ALIGNED: [(u8, &str); 5] = [(1, "u8"), (2, "u16"), (4, "u32"), (8, "u64"), (16, "u128")];
const MAX_SIZE: u8 = 200;

fn main() {
    let mut contents = String::new();

    for size in 0..=MAX_SIZE {
        for (align, aligned_ty) in ALIGNED {
            if size % align == 0 {
                let size_in_aligned_tys = size / align;

                let ident = format!("Size{size}Align{align}");

                write!(
                    &mut contents,
                    "
/// A type with a size of `{size}` bytes and alignment `{align}`.
#[repr(C, align({align}))]
pub struct {ident}([::core::mem::MaybeUninit<{aligned_ty}>; {size_in_aligned_tys}]);
impl {ident} {{
    pub const unsafe fn new_uninit() -> Self {{
        Self([::core::mem::MaybeUninit::uninit(); _])
    }}
    pub const unsafe fn new_zeroed() -> Self {{
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }}
}}

unsafe impl crate::Mimic for crate::SizeAndAlign<{size},{align}> {{ type Archetype = {ident}; }}
impl crate::private::Sealed for crate::SizeAndAlign<{size},{align}> {{}}
                    ",
                )
                .unwrap();
            }
        }
    }

    fs::write("src/types.rs", contents).unwrap()
}
