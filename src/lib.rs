#![no_std]

#[macro_export]
macro_rules! generate_mimic {
    ($( #[$meta:meta] )* $mimic_ident:ident for $ty:ty) => {
        $( #[$meta] )*
        pub struct $mimic_ident {
            _size: [
                ::core::mem::MaybeUninit<<$crate::Align<{::core::mem::align_of::<$ty>()}> as $crate::Alignment>::Archetype>;
                ::core::mem::size_of::<$ty>()
                    / ::core::mem::size_of::<<$crate::Align<{::core::mem::align_of::<$ty>()}> as $crate::Alignment>::Archetype>()],
        }

        impl $mimic_ident {
            pub const fn new() -> Self {
                Self {
                    _size: [::core::mem::MaybeUninit::uninit(); _],

                }
            }
        }

        const _: () = {
            assert!(::core::mem::size_of::<$ty>() - ::core::mem::size_of::<$mimic_ident>() == 0);
            assert!(::core::mem::align_of::<$ty>() - ::core::mem::align_of::<$mimic_ident>() == 0);
        };
    };
}

#[doc(hidden)]
pub struct Align<const N: usize>
where
    Self: Alignment;

#[doc(hidden)]
pub unsafe trait Alignment: private::Sealed {
    type Archetype;
}

#[rustfmt::skip] unsafe impl Alignment for Align<1> { type Archetype = u8; }
#[rustfmt::skip] unsafe impl Alignment for Align<2> { type Archetype = u16; }
#[rustfmt::skip] unsafe impl Alignment for Align<4> { type Archetype = u32; }
#[rustfmt::skip] unsafe impl Alignment for Align<8> { type Archetype = u64; }
#[rustfmt::skip] unsafe impl Alignment for Align<16> { type Archetype = u128; }

#[rustfmt::skip] impl private::Sealed for Align<1> { }
#[rustfmt::skip] impl private::Sealed for Align<2> { }
#[rustfmt::skip] impl private::Sealed for Align<4> { }
#[rustfmt::skip] impl private::Sealed for Align<8> { }
#[rustfmt::skip] impl private::Sealed for Align<16> { }

mod private {
    /// This trait is used internally to map an `Align<N>` to a unit
    /// struct of alignment N.
    pub trait Sealed {}
}
