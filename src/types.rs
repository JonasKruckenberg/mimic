
/// A type with a size of `0` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size0Align1([::core::mem::MaybeUninit<u8>; 0]);
impl Size0Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<0,1> { type Archetype = Size0Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<0,1> {}
                    
/// A type with a size of `0` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size0Align2([::core::mem::MaybeUninit<u16>; 0]);
impl Size0Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<0,2> { type Archetype = Size0Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<0,2> {}
                    
/// A type with a size of `0` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size0Align4([::core::mem::MaybeUninit<u32>; 0]);
impl Size0Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<0,4> { type Archetype = Size0Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<0,4> {}
                    
/// A type with a size of `0` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size0Align8([::core::mem::MaybeUninit<u64>; 0]);
impl Size0Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<0,8> { type Archetype = Size0Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<0,8> {}
                    
/// A type with a size of `0` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size0Align16([::core::mem::MaybeUninit<u128>; 0]);
impl Size0Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<0,16> { type Archetype = Size0Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<0,16> {}
                    
/// A type with a size of `1` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size1Align1([::core::mem::MaybeUninit<u8>; 1]);
impl Size1Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<1,1> { type Archetype = Size1Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<1,1> {}
                    
/// A type with a size of `2` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size2Align1([::core::mem::MaybeUninit<u8>; 2]);
impl Size2Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<2,1> { type Archetype = Size2Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<2,1> {}
                    
/// A type with a size of `2` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size2Align2([::core::mem::MaybeUninit<u16>; 1]);
impl Size2Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<2,2> { type Archetype = Size2Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<2,2> {}
                    
/// A type with a size of `3` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size3Align1([::core::mem::MaybeUninit<u8>; 3]);
impl Size3Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<3,1> { type Archetype = Size3Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<3,1> {}
                    
/// A type with a size of `4` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size4Align1([::core::mem::MaybeUninit<u8>; 4]);
impl Size4Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<4,1> { type Archetype = Size4Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<4,1> {}
                    
/// A type with a size of `4` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size4Align2([::core::mem::MaybeUninit<u16>; 2]);
impl Size4Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<4,2> { type Archetype = Size4Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<4,2> {}
                    
/// A type with a size of `4` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size4Align4([::core::mem::MaybeUninit<u32>; 1]);
impl Size4Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<4,4> { type Archetype = Size4Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<4,4> {}
                    
/// A type with a size of `5` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size5Align1([::core::mem::MaybeUninit<u8>; 5]);
impl Size5Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<5,1> { type Archetype = Size5Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<5,1> {}
                    
/// A type with a size of `6` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size6Align1([::core::mem::MaybeUninit<u8>; 6]);
impl Size6Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<6,1> { type Archetype = Size6Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<6,1> {}
                    
/// A type with a size of `6` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size6Align2([::core::mem::MaybeUninit<u16>; 3]);
impl Size6Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<6,2> { type Archetype = Size6Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<6,2> {}
                    
/// A type with a size of `7` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size7Align1([::core::mem::MaybeUninit<u8>; 7]);
impl Size7Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<7,1> { type Archetype = Size7Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<7,1> {}
                    
/// A type with a size of `8` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size8Align1([::core::mem::MaybeUninit<u8>; 8]);
impl Size8Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<8,1> { type Archetype = Size8Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<8,1> {}
                    
/// A type with a size of `8` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size8Align2([::core::mem::MaybeUninit<u16>; 4]);
impl Size8Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<8,2> { type Archetype = Size8Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<8,2> {}
                    
/// A type with a size of `8` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size8Align4([::core::mem::MaybeUninit<u32>; 2]);
impl Size8Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<8,4> { type Archetype = Size8Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<8,4> {}
                    
/// A type with a size of `8` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size8Align8([::core::mem::MaybeUninit<u64>; 1]);
impl Size8Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<8,8> { type Archetype = Size8Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<8,8> {}
                    
/// A type with a size of `9` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size9Align1([::core::mem::MaybeUninit<u8>; 9]);
impl Size9Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<9,1> { type Archetype = Size9Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<9,1> {}
                    
/// A type with a size of `10` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size10Align1([::core::mem::MaybeUninit<u8>; 10]);
impl Size10Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<10,1> { type Archetype = Size10Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<10,1> {}
                    
/// A type with a size of `10` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size10Align2([::core::mem::MaybeUninit<u16>; 5]);
impl Size10Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<10,2> { type Archetype = Size10Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<10,2> {}
                    
/// A type with a size of `11` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size11Align1([::core::mem::MaybeUninit<u8>; 11]);
impl Size11Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<11,1> { type Archetype = Size11Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<11,1> {}
                    
/// A type with a size of `12` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size12Align1([::core::mem::MaybeUninit<u8>; 12]);
impl Size12Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<12,1> { type Archetype = Size12Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<12,1> {}
                    
/// A type with a size of `12` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size12Align2([::core::mem::MaybeUninit<u16>; 6]);
impl Size12Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<12,2> { type Archetype = Size12Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<12,2> {}
                    
/// A type with a size of `12` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size12Align4([::core::mem::MaybeUninit<u32>; 3]);
impl Size12Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<12,4> { type Archetype = Size12Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<12,4> {}
                    
/// A type with a size of `13` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size13Align1([::core::mem::MaybeUninit<u8>; 13]);
impl Size13Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<13,1> { type Archetype = Size13Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<13,1> {}
                    
/// A type with a size of `14` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size14Align1([::core::mem::MaybeUninit<u8>; 14]);
impl Size14Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<14,1> { type Archetype = Size14Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<14,1> {}
                    
/// A type with a size of `14` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size14Align2([::core::mem::MaybeUninit<u16>; 7]);
impl Size14Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<14,2> { type Archetype = Size14Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<14,2> {}
                    
/// A type with a size of `15` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size15Align1([::core::mem::MaybeUninit<u8>; 15]);
impl Size15Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<15,1> { type Archetype = Size15Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<15,1> {}
                    
/// A type with a size of `16` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size16Align1([::core::mem::MaybeUninit<u8>; 16]);
impl Size16Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<16,1> { type Archetype = Size16Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<16,1> {}
                    
/// A type with a size of `16` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size16Align2([::core::mem::MaybeUninit<u16>; 8]);
impl Size16Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<16,2> { type Archetype = Size16Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<16,2> {}
                    
/// A type with a size of `16` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size16Align4([::core::mem::MaybeUninit<u32>; 4]);
impl Size16Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<16,4> { type Archetype = Size16Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<16,4> {}
                    
/// A type with a size of `16` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size16Align8([::core::mem::MaybeUninit<u64>; 2]);
impl Size16Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<16,8> { type Archetype = Size16Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<16,8> {}
                    
/// A type with a size of `16` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size16Align16([::core::mem::MaybeUninit<u128>; 1]);
impl Size16Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<16,16> { type Archetype = Size16Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<16,16> {}
                    
/// A type with a size of `17` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size17Align1([::core::mem::MaybeUninit<u8>; 17]);
impl Size17Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<17,1> { type Archetype = Size17Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<17,1> {}
                    
/// A type with a size of `18` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size18Align1([::core::mem::MaybeUninit<u8>; 18]);
impl Size18Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<18,1> { type Archetype = Size18Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<18,1> {}
                    
/// A type with a size of `18` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size18Align2([::core::mem::MaybeUninit<u16>; 9]);
impl Size18Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<18,2> { type Archetype = Size18Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<18,2> {}
                    
/// A type with a size of `19` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size19Align1([::core::mem::MaybeUninit<u8>; 19]);
impl Size19Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<19,1> { type Archetype = Size19Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<19,1> {}
                    
/// A type with a size of `20` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size20Align1([::core::mem::MaybeUninit<u8>; 20]);
impl Size20Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<20,1> { type Archetype = Size20Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<20,1> {}
                    
/// A type with a size of `20` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size20Align2([::core::mem::MaybeUninit<u16>; 10]);
impl Size20Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<20,2> { type Archetype = Size20Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<20,2> {}
                    
/// A type with a size of `20` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size20Align4([::core::mem::MaybeUninit<u32>; 5]);
impl Size20Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<20,4> { type Archetype = Size20Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<20,4> {}
                    
/// A type with a size of `21` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size21Align1([::core::mem::MaybeUninit<u8>; 21]);
impl Size21Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<21,1> { type Archetype = Size21Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<21,1> {}
                    
/// A type with a size of `22` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size22Align1([::core::mem::MaybeUninit<u8>; 22]);
impl Size22Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<22,1> { type Archetype = Size22Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<22,1> {}
                    
/// A type with a size of `22` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size22Align2([::core::mem::MaybeUninit<u16>; 11]);
impl Size22Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<22,2> { type Archetype = Size22Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<22,2> {}
                    
/// A type with a size of `23` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size23Align1([::core::mem::MaybeUninit<u8>; 23]);
impl Size23Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<23,1> { type Archetype = Size23Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<23,1> {}
                    
/// A type with a size of `24` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size24Align1([::core::mem::MaybeUninit<u8>; 24]);
impl Size24Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<24,1> { type Archetype = Size24Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<24,1> {}
                    
/// A type with a size of `24` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size24Align2([::core::mem::MaybeUninit<u16>; 12]);
impl Size24Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<24,2> { type Archetype = Size24Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<24,2> {}
                    
/// A type with a size of `24` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size24Align4([::core::mem::MaybeUninit<u32>; 6]);
impl Size24Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<24,4> { type Archetype = Size24Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<24,4> {}
                    
/// A type with a size of `24` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size24Align8([::core::mem::MaybeUninit<u64>; 3]);
impl Size24Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<24,8> { type Archetype = Size24Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<24,8> {}
                    
/// A type with a size of `25` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size25Align1([::core::mem::MaybeUninit<u8>; 25]);
impl Size25Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<25,1> { type Archetype = Size25Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<25,1> {}
                    
/// A type with a size of `26` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size26Align1([::core::mem::MaybeUninit<u8>; 26]);
impl Size26Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<26,1> { type Archetype = Size26Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<26,1> {}
                    
/// A type with a size of `26` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size26Align2([::core::mem::MaybeUninit<u16>; 13]);
impl Size26Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<26,2> { type Archetype = Size26Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<26,2> {}
                    
/// A type with a size of `27` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size27Align1([::core::mem::MaybeUninit<u8>; 27]);
impl Size27Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<27,1> { type Archetype = Size27Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<27,1> {}
                    
/// A type with a size of `28` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size28Align1([::core::mem::MaybeUninit<u8>; 28]);
impl Size28Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<28,1> { type Archetype = Size28Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<28,1> {}
                    
/// A type with a size of `28` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size28Align2([::core::mem::MaybeUninit<u16>; 14]);
impl Size28Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<28,2> { type Archetype = Size28Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<28,2> {}
                    
/// A type with a size of `28` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size28Align4([::core::mem::MaybeUninit<u32>; 7]);
impl Size28Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<28,4> { type Archetype = Size28Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<28,4> {}
                    
/// A type with a size of `29` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size29Align1([::core::mem::MaybeUninit<u8>; 29]);
impl Size29Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<29,1> { type Archetype = Size29Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<29,1> {}
                    
/// A type with a size of `30` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size30Align1([::core::mem::MaybeUninit<u8>; 30]);
impl Size30Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<30,1> { type Archetype = Size30Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<30,1> {}
                    
/// A type with a size of `30` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size30Align2([::core::mem::MaybeUninit<u16>; 15]);
impl Size30Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<30,2> { type Archetype = Size30Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<30,2> {}
                    
/// A type with a size of `31` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size31Align1([::core::mem::MaybeUninit<u8>; 31]);
impl Size31Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<31,1> { type Archetype = Size31Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<31,1> {}
                    
/// A type with a size of `32` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size32Align1([::core::mem::MaybeUninit<u8>; 32]);
impl Size32Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<32,1> { type Archetype = Size32Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<32,1> {}
                    
/// A type with a size of `32` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size32Align2([::core::mem::MaybeUninit<u16>; 16]);
impl Size32Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<32,2> { type Archetype = Size32Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<32,2> {}
                    
/// A type with a size of `32` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size32Align4([::core::mem::MaybeUninit<u32>; 8]);
impl Size32Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<32,4> { type Archetype = Size32Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<32,4> {}
                    
/// A type with a size of `32` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size32Align8([::core::mem::MaybeUninit<u64>; 4]);
impl Size32Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<32,8> { type Archetype = Size32Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<32,8> {}
                    
/// A type with a size of `32` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size32Align16([::core::mem::MaybeUninit<u128>; 2]);
impl Size32Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<32,16> { type Archetype = Size32Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<32,16> {}
                    
/// A type with a size of `33` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size33Align1([::core::mem::MaybeUninit<u8>; 33]);
impl Size33Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<33,1> { type Archetype = Size33Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<33,1> {}
                    
/// A type with a size of `34` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size34Align1([::core::mem::MaybeUninit<u8>; 34]);
impl Size34Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<34,1> { type Archetype = Size34Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<34,1> {}
                    
/// A type with a size of `34` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size34Align2([::core::mem::MaybeUninit<u16>; 17]);
impl Size34Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<34,2> { type Archetype = Size34Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<34,2> {}
                    
/// A type with a size of `35` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size35Align1([::core::mem::MaybeUninit<u8>; 35]);
impl Size35Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<35,1> { type Archetype = Size35Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<35,1> {}
                    
/// A type with a size of `36` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size36Align1([::core::mem::MaybeUninit<u8>; 36]);
impl Size36Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<36,1> { type Archetype = Size36Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<36,1> {}
                    
/// A type with a size of `36` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size36Align2([::core::mem::MaybeUninit<u16>; 18]);
impl Size36Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<36,2> { type Archetype = Size36Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<36,2> {}
                    
/// A type with a size of `36` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size36Align4([::core::mem::MaybeUninit<u32>; 9]);
impl Size36Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<36,4> { type Archetype = Size36Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<36,4> {}
                    
/// A type with a size of `37` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size37Align1([::core::mem::MaybeUninit<u8>; 37]);
impl Size37Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<37,1> { type Archetype = Size37Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<37,1> {}
                    
/// A type with a size of `38` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size38Align1([::core::mem::MaybeUninit<u8>; 38]);
impl Size38Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<38,1> { type Archetype = Size38Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<38,1> {}
                    
/// A type with a size of `38` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size38Align2([::core::mem::MaybeUninit<u16>; 19]);
impl Size38Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<38,2> { type Archetype = Size38Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<38,2> {}
                    
/// A type with a size of `39` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size39Align1([::core::mem::MaybeUninit<u8>; 39]);
impl Size39Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<39,1> { type Archetype = Size39Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<39,1> {}
                    
/// A type with a size of `40` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size40Align1([::core::mem::MaybeUninit<u8>; 40]);
impl Size40Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<40,1> { type Archetype = Size40Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<40,1> {}
                    
/// A type with a size of `40` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size40Align2([::core::mem::MaybeUninit<u16>; 20]);
impl Size40Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<40,2> { type Archetype = Size40Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<40,2> {}
                    
/// A type with a size of `40` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size40Align4([::core::mem::MaybeUninit<u32>; 10]);
impl Size40Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<40,4> { type Archetype = Size40Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<40,4> {}
                    
/// A type with a size of `40` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size40Align8([::core::mem::MaybeUninit<u64>; 5]);
impl Size40Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<40,8> { type Archetype = Size40Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<40,8> {}
                    
/// A type with a size of `41` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size41Align1([::core::mem::MaybeUninit<u8>; 41]);
impl Size41Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<41,1> { type Archetype = Size41Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<41,1> {}
                    
/// A type with a size of `42` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size42Align1([::core::mem::MaybeUninit<u8>; 42]);
impl Size42Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<42,1> { type Archetype = Size42Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<42,1> {}
                    
/// A type with a size of `42` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size42Align2([::core::mem::MaybeUninit<u16>; 21]);
impl Size42Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<42,2> { type Archetype = Size42Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<42,2> {}
                    
/// A type with a size of `43` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size43Align1([::core::mem::MaybeUninit<u8>; 43]);
impl Size43Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<43,1> { type Archetype = Size43Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<43,1> {}
                    
/// A type with a size of `44` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size44Align1([::core::mem::MaybeUninit<u8>; 44]);
impl Size44Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<44,1> { type Archetype = Size44Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<44,1> {}
                    
/// A type with a size of `44` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size44Align2([::core::mem::MaybeUninit<u16>; 22]);
impl Size44Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<44,2> { type Archetype = Size44Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<44,2> {}
                    
/// A type with a size of `44` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size44Align4([::core::mem::MaybeUninit<u32>; 11]);
impl Size44Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<44,4> { type Archetype = Size44Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<44,4> {}
                    
/// A type with a size of `45` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size45Align1([::core::mem::MaybeUninit<u8>; 45]);
impl Size45Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<45,1> { type Archetype = Size45Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<45,1> {}
                    
/// A type with a size of `46` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size46Align1([::core::mem::MaybeUninit<u8>; 46]);
impl Size46Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<46,1> { type Archetype = Size46Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<46,1> {}
                    
/// A type with a size of `46` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size46Align2([::core::mem::MaybeUninit<u16>; 23]);
impl Size46Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<46,2> { type Archetype = Size46Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<46,2> {}
                    
/// A type with a size of `47` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size47Align1([::core::mem::MaybeUninit<u8>; 47]);
impl Size47Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<47,1> { type Archetype = Size47Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<47,1> {}
                    
/// A type with a size of `48` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size48Align1([::core::mem::MaybeUninit<u8>; 48]);
impl Size48Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<48,1> { type Archetype = Size48Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<48,1> {}
                    
/// A type with a size of `48` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size48Align2([::core::mem::MaybeUninit<u16>; 24]);
impl Size48Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<48,2> { type Archetype = Size48Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<48,2> {}
                    
/// A type with a size of `48` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size48Align4([::core::mem::MaybeUninit<u32>; 12]);
impl Size48Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<48,4> { type Archetype = Size48Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<48,4> {}
                    
/// A type with a size of `48` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size48Align8([::core::mem::MaybeUninit<u64>; 6]);
impl Size48Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<48,8> { type Archetype = Size48Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<48,8> {}
                    
/// A type with a size of `48` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size48Align16([::core::mem::MaybeUninit<u128>; 3]);
impl Size48Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<48,16> { type Archetype = Size48Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<48,16> {}
                    
/// A type with a size of `49` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size49Align1([::core::mem::MaybeUninit<u8>; 49]);
impl Size49Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<49,1> { type Archetype = Size49Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<49,1> {}
                    
/// A type with a size of `50` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size50Align1([::core::mem::MaybeUninit<u8>; 50]);
impl Size50Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<50,1> { type Archetype = Size50Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<50,1> {}
                    
/// A type with a size of `50` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size50Align2([::core::mem::MaybeUninit<u16>; 25]);
impl Size50Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<50,2> { type Archetype = Size50Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<50,2> {}
                    
/// A type with a size of `51` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size51Align1([::core::mem::MaybeUninit<u8>; 51]);
impl Size51Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<51,1> { type Archetype = Size51Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<51,1> {}
                    
/// A type with a size of `52` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size52Align1([::core::mem::MaybeUninit<u8>; 52]);
impl Size52Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<52,1> { type Archetype = Size52Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<52,1> {}
                    
/// A type with a size of `52` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size52Align2([::core::mem::MaybeUninit<u16>; 26]);
impl Size52Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<52,2> { type Archetype = Size52Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<52,2> {}
                    
/// A type with a size of `52` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size52Align4([::core::mem::MaybeUninit<u32>; 13]);
impl Size52Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<52,4> { type Archetype = Size52Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<52,4> {}
                    
/// A type with a size of `53` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size53Align1([::core::mem::MaybeUninit<u8>; 53]);
impl Size53Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<53,1> { type Archetype = Size53Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<53,1> {}
                    
/// A type with a size of `54` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size54Align1([::core::mem::MaybeUninit<u8>; 54]);
impl Size54Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<54,1> { type Archetype = Size54Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<54,1> {}
                    
/// A type with a size of `54` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size54Align2([::core::mem::MaybeUninit<u16>; 27]);
impl Size54Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<54,2> { type Archetype = Size54Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<54,2> {}
                    
/// A type with a size of `55` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size55Align1([::core::mem::MaybeUninit<u8>; 55]);
impl Size55Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<55,1> { type Archetype = Size55Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<55,1> {}
                    
/// A type with a size of `56` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size56Align1([::core::mem::MaybeUninit<u8>; 56]);
impl Size56Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<56,1> { type Archetype = Size56Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<56,1> {}
                    
/// A type with a size of `56` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size56Align2([::core::mem::MaybeUninit<u16>; 28]);
impl Size56Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<56,2> { type Archetype = Size56Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<56,2> {}
                    
/// A type with a size of `56` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size56Align4([::core::mem::MaybeUninit<u32>; 14]);
impl Size56Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<56,4> { type Archetype = Size56Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<56,4> {}
                    
/// A type with a size of `56` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size56Align8([::core::mem::MaybeUninit<u64>; 7]);
impl Size56Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<56,8> { type Archetype = Size56Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<56,8> {}
                    
/// A type with a size of `57` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size57Align1([::core::mem::MaybeUninit<u8>; 57]);
impl Size57Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<57,1> { type Archetype = Size57Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<57,1> {}
                    
/// A type with a size of `58` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size58Align1([::core::mem::MaybeUninit<u8>; 58]);
impl Size58Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<58,1> { type Archetype = Size58Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<58,1> {}
                    
/// A type with a size of `58` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size58Align2([::core::mem::MaybeUninit<u16>; 29]);
impl Size58Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<58,2> { type Archetype = Size58Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<58,2> {}
                    
/// A type with a size of `59` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size59Align1([::core::mem::MaybeUninit<u8>; 59]);
impl Size59Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<59,1> { type Archetype = Size59Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<59,1> {}
                    
/// A type with a size of `60` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size60Align1([::core::mem::MaybeUninit<u8>; 60]);
impl Size60Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<60,1> { type Archetype = Size60Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<60,1> {}
                    
/// A type with a size of `60` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size60Align2([::core::mem::MaybeUninit<u16>; 30]);
impl Size60Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<60,2> { type Archetype = Size60Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<60,2> {}
                    
/// A type with a size of `60` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size60Align4([::core::mem::MaybeUninit<u32>; 15]);
impl Size60Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<60,4> { type Archetype = Size60Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<60,4> {}
                    
/// A type with a size of `61` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size61Align1([::core::mem::MaybeUninit<u8>; 61]);
impl Size61Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<61,1> { type Archetype = Size61Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<61,1> {}
                    
/// A type with a size of `62` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size62Align1([::core::mem::MaybeUninit<u8>; 62]);
impl Size62Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<62,1> { type Archetype = Size62Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<62,1> {}
                    
/// A type with a size of `62` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size62Align2([::core::mem::MaybeUninit<u16>; 31]);
impl Size62Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<62,2> { type Archetype = Size62Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<62,2> {}
                    
/// A type with a size of `63` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size63Align1([::core::mem::MaybeUninit<u8>; 63]);
impl Size63Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<63,1> { type Archetype = Size63Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<63,1> {}
                    
/// A type with a size of `64` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size64Align1([::core::mem::MaybeUninit<u8>; 64]);
impl Size64Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<64,1> { type Archetype = Size64Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<64,1> {}
                    
/// A type with a size of `64` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size64Align2([::core::mem::MaybeUninit<u16>; 32]);
impl Size64Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<64,2> { type Archetype = Size64Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<64,2> {}
                    
/// A type with a size of `64` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size64Align4([::core::mem::MaybeUninit<u32>; 16]);
impl Size64Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<64,4> { type Archetype = Size64Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<64,4> {}
                    
/// A type with a size of `64` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size64Align8([::core::mem::MaybeUninit<u64>; 8]);
impl Size64Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<64,8> { type Archetype = Size64Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<64,8> {}
                    
/// A type with a size of `64` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size64Align16([::core::mem::MaybeUninit<u128>; 4]);
impl Size64Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<64,16> { type Archetype = Size64Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<64,16> {}
                    
/// A type with a size of `65` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size65Align1([::core::mem::MaybeUninit<u8>; 65]);
impl Size65Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<65,1> { type Archetype = Size65Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<65,1> {}
                    
/// A type with a size of `66` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size66Align1([::core::mem::MaybeUninit<u8>; 66]);
impl Size66Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<66,1> { type Archetype = Size66Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<66,1> {}
                    
/// A type with a size of `66` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size66Align2([::core::mem::MaybeUninit<u16>; 33]);
impl Size66Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<66,2> { type Archetype = Size66Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<66,2> {}
                    
/// A type with a size of `67` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size67Align1([::core::mem::MaybeUninit<u8>; 67]);
impl Size67Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<67,1> { type Archetype = Size67Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<67,1> {}
                    
/// A type with a size of `68` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size68Align1([::core::mem::MaybeUninit<u8>; 68]);
impl Size68Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<68,1> { type Archetype = Size68Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<68,1> {}
                    
/// A type with a size of `68` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size68Align2([::core::mem::MaybeUninit<u16>; 34]);
impl Size68Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<68,2> { type Archetype = Size68Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<68,2> {}
                    
/// A type with a size of `68` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size68Align4([::core::mem::MaybeUninit<u32>; 17]);
impl Size68Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<68,4> { type Archetype = Size68Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<68,4> {}
                    
/// A type with a size of `69` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size69Align1([::core::mem::MaybeUninit<u8>; 69]);
impl Size69Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<69,1> { type Archetype = Size69Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<69,1> {}
                    
/// A type with a size of `70` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size70Align1([::core::mem::MaybeUninit<u8>; 70]);
impl Size70Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<70,1> { type Archetype = Size70Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<70,1> {}
                    
/// A type with a size of `70` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size70Align2([::core::mem::MaybeUninit<u16>; 35]);
impl Size70Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<70,2> { type Archetype = Size70Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<70,2> {}
                    
/// A type with a size of `71` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size71Align1([::core::mem::MaybeUninit<u8>; 71]);
impl Size71Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<71,1> { type Archetype = Size71Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<71,1> {}
                    
/// A type with a size of `72` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size72Align1([::core::mem::MaybeUninit<u8>; 72]);
impl Size72Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<72,1> { type Archetype = Size72Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<72,1> {}
                    
/// A type with a size of `72` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size72Align2([::core::mem::MaybeUninit<u16>; 36]);
impl Size72Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<72,2> { type Archetype = Size72Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<72,2> {}
                    
/// A type with a size of `72` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size72Align4([::core::mem::MaybeUninit<u32>; 18]);
impl Size72Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<72,4> { type Archetype = Size72Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<72,4> {}
                    
/// A type with a size of `72` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size72Align8([::core::mem::MaybeUninit<u64>; 9]);
impl Size72Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<72,8> { type Archetype = Size72Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<72,8> {}
                    
/// A type with a size of `73` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size73Align1([::core::mem::MaybeUninit<u8>; 73]);
impl Size73Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<73,1> { type Archetype = Size73Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<73,1> {}
                    
/// A type with a size of `74` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size74Align1([::core::mem::MaybeUninit<u8>; 74]);
impl Size74Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<74,1> { type Archetype = Size74Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<74,1> {}
                    
/// A type with a size of `74` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size74Align2([::core::mem::MaybeUninit<u16>; 37]);
impl Size74Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<74,2> { type Archetype = Size74Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<74,2> {}
                    
/// A type with a size of `75` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size75Align1([::core::mem::MaybeUninit<u8>; 75]);
impl Size75Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<75,1> { type Archetype = Size75Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<75,1> {}
                    
/// A type with a size of `76` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size76Align1([::core::mem::MaybeUninit<u8>; 76]);
impl Size76Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<76,1> { type Archetype = Size76Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<76,1> {}
                    
/// A type with a size of `76` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size76Align2([::core::mem::MaybeUninit<u16>; 38]);
impl Size76Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<76,2> { type Archetype = Size76Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<76,2> {}
                    
/// A type with a size of `76` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size76Align4([::core::mem::MaybeUninit<u32>; 19]);
impl Size76Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<76,4> { type Archetype = Size76Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<76,4> {}
                    
/// A type with a size of `77` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size77Align1([::core::mem::MaybeUninit<u8>; 77]);
impl Size77Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<77,1> { type Archetype = Size77Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<77,1> {}
                    
/// A type with a size of `78` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size78Align1([::core::mem::MaybeUninit<u8>; 78]);
impl Size78Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<78,1> { type Archetype = Size78Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<78,1> {}
                    
/// A type with a size of `78` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size78Align2([::core::mem::MaybeUninit<u16>; 39]);
impl Size78Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<78,2> { type Archetype = Size78Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<78,2> {}
                    
/// A type with a size of `79` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size79Align1([::core::mem::MaybeUninit<u8>; 79]);
impl Size79Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<79,1> { type Archetype = Size79Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<79,1> {}
                    
/// A type with a size of `80` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size80Align1([::core::mem::MaybeUninit<u8>; 80]);
impl Size80Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<80,1> { type Archetype = Size80Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<80,1> {}
                    
/// A type with a size of `80` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size80Align2([::core::mem::MaybeUninit<u16>; 40]);
impl Size80Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<80,2> { type Archetype = Size80Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<80,2> {}
                    
/// A type with a size of `80` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size80Align4([::core::mem::MaybeUninit<u32>; 20]);
impl Size80Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<80,4> { type Archetype = Size80Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<80,4> {}
                    
/// A type with a size of `80` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size80Align8([::core::mem::MaybeUninit<u64>; 10]);
impl Size80Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<80,8> { type Archetype = Size80Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<80,8> {}
                    
/// A type with a size of `80` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size80Align16([::core::mem::MaybeUninit<u128>; 5]);
impl Size80Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<80,16> { type Archetype = Size80Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<80,16> {}
                    
/// A type with a size of `81` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size81Align1([::core::mem::MaybeUninit<u8>; 81]);
impl Size81Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<81,1> { type Archetype = Size81Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<81,1> {}
                    
/// A type with a size of `82` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size82Align1([::core::mem::MaybeUninit<u8>; 82]);
impl Size82Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<82,1> { type Archetype = Size82Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<82,1> {}
                    
/// A type with a size of `82` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size82Align2([::core::mem::MaybeUninit<u16>; 41]);
impl Size82Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<82,2> { type Archetype = Size82Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<82,2> {}
                    
/// A type with a size of `83` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size83Align1([::core::mem::MaybeUninit<u8>; 83]);
impl Size83Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<83,1> { type Archetype = Size83Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<83,1> {}
                    
/// A type with a size of `84` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size84Align1([::core::mem::MaybeUninit<u8>; 84]);
impl Size84Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<84,1> { type Archetype = Size84Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<84,1> {}
                    
/// A type with a size of `84` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size84Align2([::core::mem::MaybeUninit<u16>; 42]);
impl Size84Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<84,2> { type Archetype = Size84Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<84,2> {}
                    
/// A type with a size of `84` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size84Align4([::core::mem::MaybeUninit<u32>; 21]);
impl Size84Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<84,4> { type Archetype = Size84Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<84,4> {}
                    
/// A type with a size of `85` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size85Align1([::core::mem::MaybeUninit<u8>; 85]);
impl Size85Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<85,1> { type Archetype = Size85Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<85,1> {}
                    
/// A type with a size of `86` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size86Align1([::core::mem::MaybeUninit<u8>; 86]);
impl Size86Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<86,1> { type Archetype = Size86Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<86,1> {}
                    
/// A type with a size of `86` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size86Align2([::core::mem::MaybeUninit<u16>; 43]);
impl Size86Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<86,2> { type Archetype = Size86Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<86,2> {}
                    
/// A type with a size of `87` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size87Align1([::core::mem::MaybeUninit<u8>; 87]);
impl Size87Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<87,1> { type Archetype = Size87Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<87,1> {}
                    
/// A type with a size of `88` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size88Align1([::core::mem::MaybeUninit<u8>; 88]);
impl Size88Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<88,1> { type Archetype = Size88Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<88,1> {}
                    
/// A type with a size of `88` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size88Align2([::core::mem::MaybeUninit<u16>; 44]);
impl Size88Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<88,2> { type Archetype = Size88Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<88,2> {}
                    
/// A type with a size of `88` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size88Align4([::core::mem::MaybeUninit<u32>; 22]);
impl Size88Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<88,4> { type Archetype = Size88Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<88,4> {}
                    
/// A type with a size of `88` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size88Align8([::core::mem::MaybeUninit<u64>; 11]);
impl Size88Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<88,8> { type Archetype = Size88Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<88,8> {}
                    
/// A type with a size of `89` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size89Align1([::core::mem::MaybeUninit<u8>; 89]);
impl Size89Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<89,1> { type Archetype = Size89Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<89,1> {}
                    
/// A type with a size of `90` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size90Align1([::core::mem::MaybeUninit<u8>; 90]);
impl Size90Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<90,1> { type Archetype = Size90Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<90,1> {}
                    
/// A type with a size of `90` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size90Align2([::core::mem::MaybeUninit<u16>; 45]);
impl Size90Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<90,2> { type Archetype = Size90Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<90,2> {}
                    
/// A type with a size of `91` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size91Align1([::core::mem::MaybeUninit<u8>; 91]);
impl Size91Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<91,1> { type Archetype = Size91Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<91,1> {}
                    
/// A type with a size of `92` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size92Align1([::core::mem::MaybeUninit<u8>; 92]);
impl Size92Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<92,1> { type Archetype = Size92Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<92,1> {}
                    
/// A type with a size of `92` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size92Align2([::core::mem::MaybeUninit<u16>; 46]);
impl Size92Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<92,2> { type Archetype = Size92Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<92,2> {}
                    
/// A type with a size of `92` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size92Align4([::core::mem::MaybeUninit<u32>; 23]);
impl Size92Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<92,4> { type Archetype = Size92Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<92,4> {}
                    
/// A type with a size of `93` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size93Align1([::core::mem::MaybeUninit<u8>; 93]);
impl Size93Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<93,1> { type Archetype = Size93Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<93,1> {}
                    
/// A type with a size of `94` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size94Align1([::core::mem::MaybeUninit<u8>; 94]);
impl Size94Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<94,1> { type Archetype = Size94Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<94,1> {}
                    
/// A type with a size of `94` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size94Align2([::core::mem::MaybeUninit<u16>; 47]);
impl Size94Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<94,2> { type Archetype = Size94Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<94,2> {}
                    
/// A type with a size of `95` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size95Align1([::core::mem::MaybeUninit<u8>; 95]);
impl Size95Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<95,1> { type Archetype = Size95Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<95,1> {}
                    
/// A type with a size of `96` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size96Align1([::core::mem::MaybeUninit<u8>; 96]);
impl Size96Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<96,1> { type Archetype = Size96Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<96,1> {}
                    
/// A type with a size of `96` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size96Align2([::core::mem::MaybeUninit<u16>; 48]);
impl Size96Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<96,2> { type Archetype = Size96Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<96,2> {}
                    
/// A type with a size of `96` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size96Align4([::core::mem::MaybeUninit<u32>; 24]);
impl Size96Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<96,4> { type Archetype = Size96Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<96,4> {}
                    
/// A type with a size of `96` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size96Align8([::core::mem::MaybeUninit<u64>; 12]);
impl Size96Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<96,8> { type Archetype = Size96Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<96,8> {}
                    
/// A type with a size of `96` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size96Align16([::core::mem::MaybeUninit<u128>; 6]);
impl Size96Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<96,16> { type Archetype = Size96Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<96,16> {}
                    
/// A type with a size of `97` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size97Align1([::core::mem::MaybeUninit<u8>; 97]);
impl Size97Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<97,1> { type Archetype = Size97Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<97,1> {}
                    
/// A type with a size of `98` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size98Align1([::core::mem::MaybeUninit<u8>; 98]);
impl Size98Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<98,1> { type Archetype = Size98Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<98,1> {}
                    
/// A type with a size of `98` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size98Align2([::core::mem::MaybeUninit<u16>; 49]);
impl Size98Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<98,2> { type Archetype = Size98Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<98,2> {}
                    
/// A type with a size of `99` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size99Align1([::core::mem::MaybeUninit<u8>; 99]);
impl Size99Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<99,1> { type Archetype = Size99Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<99,1> {}
                    
/// A type with a size of `100` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size100Align1([::core::mem::MaybeUninit<u8>; 100]);
impl Size100Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<100,1> { type Archetype = Size100Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<100,1> {}
                    
/// A type with a size of `100` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size100Align2([::core::mem::MaybeUninit<u16>; 50]);
impl Size100Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<100,2> { type Archetype = Size100Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<100,2> {}
                    
/// A type with a size of `100` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size100Align4([::core::mem::MaybeUninit<u32>; 25]);
impl Size100Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<100,4> { type Archetype = Size100Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<100,4> {}
                    