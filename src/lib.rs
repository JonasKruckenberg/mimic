#![no_std]

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

#[macro_export]
macro_rules! generate_mimic {
    ($( #[$meta:meta] )* $mimic_ident:ident for $ty:ty) => {
        $( #[$meta] )*
        pub struct $mimic_ident {
            _size: [::core::mem::MaybeUninit<u8>; ::core::mem::size_of::<$ty>()],
            _align: $crate::Align<{ ::core::mem::align_of::<$ty>() }>,
        }

        impl $mimic_ident {
            pub const fn new() -> Self {
                Self {
                    _size: [::core::mem::MaybeUninit::uninit(); _],
                    _align: $crate::Align::new(),
                }
            }
        }

        const _: () = {
            assert!(::core::mem::size_of::<$ty>() - ::core::mem::size_of::<$mimic_ident>() == 0);
            assert!(::core::mem::align_of::<$ty>() - ::core::mem::align_of::<$mimic_ident>() == 0);
        };
    };
}

/// A zero-sized-type aligned to `N`. Mimic types will contain a field `Align<N>`
/// to represent their types alignment.
///
/// ```
/// use mimic::Align;
/// use core::mem::{align_of, align_of_val};
///
/// assert_eq!(align_of::<Align<1>>(), 1);
/// assert_eq!(align_of::<Align<2>>(), 2);
/// assert_eq!(align_of::<Align<4>>(), 4);
///
/// const FOO_ALIGN: usize = 8;
///
/// #[repr(C)]
/// struct Foo {
///     _align: Align<FOO_ALIGN>,
/// }
///
/// let foo: Foo = Foo { _align: Align::new() };
///
/// assert_eq!(align_of_val(&foo), 8);
/// ```
// NB: `Eq` and `PartialEq` are derived so that `Align` also
// implements `StructuralEq` and `PartialStructuralEq`, which makes
// `Align` usable as a const generic type. Just in case.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct Align<const N: usize>([<Self as private::Sealed>::Archetype; 0])
where
    Self: Alignment;

impl<const N: usize> Align<N>
where
    Self: Alignment,
{
    /// Create an new instance of `Align<N>`.
    /// ```
    /// use mimic::Align;
    /// use core::mem::{align_of, align_of_val};
    ///
    /// #[repr(C)]
    /// struct Foo {
    ///     _align: Align<8>,
    /// }
    ///
    /// let foo: Foo = Foo { _align: Align::new() };
    ///
    /// assert_eq!(align_of_val(&foo), 8);
    /// ```
    pub const fn new() -> Self {
        Self([])
    }
}

#[doc(hidden)]
pub unsafe trait Alignment: private::Sealed {}

mod private {
    /// This trait is used internally to map an `Align<N>` to a unit
    /// struct of alignment N.
    pub trait Sealed {
        /// A zero-sized type of particular alignment.
        type Archetype: Copy + Eq + PartialEq + Send + Sync + Unpin;
    }

    #[rustfmt::skip] impl Sealed for super::Align<        1> { type Archetype = Align1;         }
    #[rustfmt::skip] impl Sealed for super::Align<        2> { type Archetype = Align2;         }
    #[rustfmt::skip] impl Sealed for super::Align<        4> { type Archetype = Align4;         }
    #[rustfmt::skip] impl Sealed for super::Align<        8> { type Archetype = Align8;         }
    #[rustfmt::skip] impl Sealed for super::Align<       16> { type Archetype = Align16;        }
    #[rustfmt::skip] impl Sealed for super::Align<       32> { type Archetype = Align32;        }
    #[rustfmt::skip] impl Sealed for super::Align<       64> { type Archetype = Align64;        }
    #[rustfmt::skip] impl Sealed for super::Align<      128> { type Archetype = Align128;       }
    #[rustfmt::skip] impl Sealed for super::Align<      256> { type Archetype = Align256;       }
    #[rustfmt::skip] impl Sealed for super::Align<      512> { type Archetype = Align512;       }
    #[rustfmt::skip] impl Sealed for super::Align<     1024> { type Archetype = Align1024;      }
    #[rustfmt::skip] impl Sealed for super::Align<     2048> { type Archetype = Align2048;      }
    #[rustfmt::skip] impl Sealed for super::Align<     4096> { type Archetype = Align4096;      }
    #[rustfmt::skip] impl Sealed for super::Align<     8192> { type Archetype = Align8192;      }
    #[rustfmt::skip] impl Sealed for super::Align<    16384> { type Archetype = Align16384;     }
    #[rustfmt::skip] impl Sealed for super::Align<    32768> { type Archetype = Align32768;     }
    #[rustfmt::skip] impl Sealed for super::Align<    65536> { type Archetype = Align65536;     }
    #[rustfmt::skip] impl Sealed for super::Align<   131072> { type Archetype = Align131072;    }
    #[rustfmt::skip] impl Sealed for super::Align<   262144> { type Archetype = Align262144;    }
    #[rustfmt::skip] impl Sealed for super::Align<   524288> { type Archetype = Align524288;    }
    #[rustfmt::skip] impl Sealed for super::Align<  1048576> { type Archetype = Align1048576;   }
    #[rustfmt::skip] impl Sealed for super::Align<  2097152> { type Archetype = Align2097152;   }
    #[rustfmt::skip] impl Sealed for super::Align<  4194304> { type Archetype = Align4194304;   }
    #[rustfmt::skip] impl Sealed for super::Align<  8388608> { type Archetype = Align8388608;   }
    #[rustfmt::skip] impl Sealed for super::Align< 16777216> { type Archetype = Align16777216;  }
    #[rustfmt::skip] impl Sealed for super::Align< 33554432> { type Archetype = Align33554432;  }
    #[rustfmt::skip] impl Sealed for super::Align< 67108864> { type Archetype = Align67108864;  }
    #[rustfmt::skip] impl Sealed for super::Align<134217728> { type Archetype = Align134217728; }
    #[rustfmt::skip] impl Sealed for super::Align<268435456> { type Archetype = Align268435456; }
    #[rustfmt::skip] impl Sealed for super::Align<536870912> { type Archetype = Align536870912; }

    // NB: It'd be great if these could be void enums, as doing so
    // greatly simplifies the expansion of derived traits.
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(        1))] pub struct Align1         (u8);
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(        2))] pub struct Align2         (u16);
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(        4))] pub struct Align4         (u32);
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(        8))] pub struct Align8         (u64);
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(       16))] pub struct Align16        (u128);
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(       32))] pub struct Align32        {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(       64))] pub struct Align64        {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(      128))] pub struct Align128       {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(      256))] pub struct Align256       {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(      512))] pub struct Align512       {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(     1024))] pub struct Align1024      {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(     2048))] pub struct Align2048      {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(     4096))] pub struct Align4096      {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(     8192))] pub struct Align8192      {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(    16384))] pub struct Align16384     {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(    32768))] pub struct Align32768     {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(    65536))] pub struct Align65536     {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(   131072))] pub struct Align131072    {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(   262144))] pub struct Align262144    {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(   524288))] pub struct Align524288    {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(  1048576))] pub struct Align1048576   {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(  2097152))] pub struct Align2097152   {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(  4194304))] pub struct Align4194304   {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(  8388608))] pub struct Align8388608   {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align( 16777216))] pub struct Align16777216  {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align( 33554432))] pub struct Align33554432  {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align( 67108864))] pub struct Align67108864  {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(134217728))] pub struct Align134217728 {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(268435456))] pub struct Align268435456 {}
    #[rustfmt::skip] #[derive(Copy, Clone, Eq, PartialEq)] #[repr(align(536870912))] pub struct Align536870912 {}
}

// NB: While these impls could be reduced to a single:
//    unsafe impl<const N: usize> Alignment for Align<N>
//    where
//        Self: private::Sealed
//    {}
// …leaving them enumerated makes explicit what alignments are valid.
#[rustfmt::skip] unsafe impl Alignment for Align<        1> {}
#[rustfmt::skip] unsafe impl Alignment for Align<        2> {}
#[rustfmt::skip] unsafe impl Alignment for Align<        4> {}
#[rustfmt::skip] unsafe impl Alignment for Align<        8> {}
#[rustfmt::skip] unsafe impl Alignment for Align<       16> {}
#[rustfmt::skip] unsafe impl Alignment for Align<       32> {}
#[rustfmt::skip] unsafe impl Alignment for Align<       64> {}
#[rustfmt::skip] unsafe impl Alignment for Align<      128> {}
#[rustfmt::skip] unsafe impl Alignment for Align<      256> {}
#[rustfmt::skip] unsafe impl Alignment for Align<      512> {}
#[rustfmt::skip] unsafe impl Alignment for Align<     1024> {}
#[rustfmt::skip] unsafe impl Alignment for Align<     2048> {}
#[rustfmt::skip] unsafe impl Alignment for Align<     4096> {}
#[rustfmt::skip] unsafe impl Alignment for Align<     8192> {}
#[rustfmt::skip] unsafe impl Alignment for Align<    16384> {}
#[rustfmt::skip] unsafe impl Alignment for Align<    32768> {}
#[rustfmt::skip] unsafe impl Alignment for Align<    65536> {}
#[rustfmt::skip] unsafe impl Alignment for Align<   131072> {}
#[rustfmt::skip] unsafe impl Alignment for Align<   262144> {}
#[rustfmt::skip] unsafe impl Alignment for Align<   524288> {}
#[rustfmt::skip] unsafe impl Alignment for Align<  1048576> {}
#[rustfmt::skip] unsafe impl Alignment for Align<  2097152> {}
#[rustfmt::skip] unsafe impl Alignment for Align<  4194304> {}
#[rustfmt::skip] unsafe impl Alignment for Align<  8388608> {}
#[rustfmt::skip] unsafe impl Alignment for Align< 16777216> {}
#[rustfmt::skip] unsafe impl Alignment for Align< 33554432> {}
#[rustfmt::skip] unsafe impl Alignment for Align< 67108864> {}
#[rustfmt::skip] unsafe impl Alignment for Align<134217728> {}
#[rustfmt::skip] unsafe impl Alignment for Align<268435456> {}
#[rustfmt::skip] unsafe impl Alignment for Align<536870912> {}

// NB: These traits are implemented explicitly, rather than derived,
// because their implementations do not depend on `Align`'s field.

impl<const N: usize> Copy for Align<N> where Self: Alignment {}

impl<const N: usize> Clone for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize> fmt::Debug for Align<N>
where
    Self: Alignment,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(core::any::type_name::<Self>())
    }
}

impl<const N: usize> Default for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn default() -> Self {
        Self([])
    }
}

impl<const N: usize> Hash for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl<const N: usize> Ord for Align<N>
where
    Self: Alignment,
{
    #[inline(always)]
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<const N: usize> PartialOrd<Self> for Align<N>
where
    Self: Alignment,
{
    #[expect(
        clippy::non_canonical_partial_ord_impl,
        reason = "Align<N>s are always equal if their Ns are equal"
    )]
    #[inline(always)]
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}
