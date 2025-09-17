#![no_std]

#[macro_export]
macro_rules! impl_mimic {
    ($ty:ty) => {
        impl $ty {
            pub const unsafe fn from_mimic(
                mimic: <$crate::SizeAndAlign<
                    { ::core::mem::size_of::<$ty>() },
                    { ::core::mem::align_of::<$ty>() },
                > as $crate::Mimic>::Archetype,
            ) -> Self {
                unsafe { ::core::mem::transmute(mimic) }
            }

            pub const fn into_mimic(
                self,
            ) -> <$crate::SizeAndAlign<
                { ::core::mem::size_of::<$ty>() },
                { ::core::mem::align_of::<$ty>() },
            > as $crate::Mimic>::Archetype {
                unsafe { ::core::mem::transmute(self) }
            }
        }

        const _: () = {
            assert!(
                ::core::mem::size_of::<$ty>()
                    - ::core::mem::size_of::<
                        <$crate::SizeAndAlign<
                            { ::core::mem::size_of::<$ty>() },
                            { ::core::mem::align_of::<$ty>() },
                        > as $crate::Mimic>::Archetype,
                    >()
                    == 0
            );

            assert!(
                ::core::mem::align_of::<$ty>()
                    - ::core::mem::align_of::<
                        <$crate::SizeAndAlign<
                            { ::core::mem::size_of::<$ty>() },
                            { ::core::mem::align_of::<$ty>() },
                        > as $crate::Mimic>::Archetype,
                    >()
                    == 0
            );
        };
    };
}

#[doc(hidden)]
#[repr(transparent)]
pub struct SizeAndAlign<const SIZE: usize, const ALIGN: usize>
where
    Self: Mimic;

#[doc(hidden)]
pub unsafe trait Mimic: private::Sealed {
    type Archetype;
}

mod types;
pub use types::*;

mod private {
    /// This trait is used internally to map an `Align<N>` to a unit
    /// struct of alignment N.
    pub trait Sealed {}
}
