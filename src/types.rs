
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
                    
/// A type with a size of `101` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size101Align1([::core::mem::MaybeUninit<u8>; 101]);
impl Size101Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<101,1> { type Archetype = Size101Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<101,1> {}
                    
/// A type with a size of `102` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size102Align1([::core::mem::MaybeUninit<u8>; 102]);
impl Size102Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<102,1> { type Archetype = Size102Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<102,1> {}
                    
/// A type with a size of `102` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size102Align2([::core::mem::MaybeUninit<u16>; 51]);
impl Size102Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<102,2> { type Archetype = Size102Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<102,2> {}
                    
/// A type with a size of `103` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size103Align1([::core::mem::MaybeUninit<u8>; 103]);
impl Size103Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<103,1> { type Archetype = Size103Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<103,1> {}
                    
/// A type with a size of `104` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size104Align1([::core::mem::MaybeUninit<u8>; 104]);
impl Size104Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<104,1> { type Archetype = Size104Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<104,1> {}
                    
/// A type with a size of `104` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size104Align2([::core::mem::MaybeUninit<u16>; 52]);
impl Size104Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<104,2> { type Archetype = Size104Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<104,2> {}
                    
/// A type with a size of `104` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size104Align4([::core::mem::MaybeUninit<u32>; 26]);
impl Size104Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<104,4> { type Archetype = Size104Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<104,4> {}
                    
/// A type with a size of `104` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size104Align8([::core::mem::MaybeUninit<u64>; 13]);
impl Size104Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<104,8> { type Archetype = Size104Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<104,8> {}
                    
/// A type with a size of `105` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size105Align1([::core::mem::MaybeUninit<u8>; 105]);
impl Size105Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<105,1> { type Archetype = Size105Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<105,1> {}
                    
/// A type with a size of `106` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size106Align1([::core::mem::MaybeUninit<u8>; 106]);
impl Size106Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<106,1> { type Archetype = Size106Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<106,1> {}
                    
/// A type with a size of `106` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size106Align2([::core::mem::MaybeUninit<u16>; 53]);
impl Size106Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<106,2> { type Archetype = Size106Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<106,2> {}
                    
/// A type with a size of `107` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size107Align1([::core::mem::MaybeUninit<u8>; 107]);
impl Size107Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<107,1> { type Archetype = Size107Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<107,1> {}
                    
/// A type with a size of `108` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size108Align1([::core::mem::MaybeUninit<u8>; 108]);
impl Size108Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<108,1> { type Archetype = Size108Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<108,1> {}
                    
/// A type with a size of `108` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size108Align2([::core::mem::MaybeUninit<u16>; 54]);
impl Size108Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<108,2> { type Archetype = Size108Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<108,2> {}
                    
/// A type with a size of `108` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size108Align4([::core::mem::MaybeUninit<u32>; 27]);
impl Size108Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<108,4> { type Archetype = Size108Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<108,4> {}
                    
/// A type with a size of `109` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size109Align1([::core::mem::MaybeUninit<u8>; 109]);
impl Size109Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<109,1> { type Archetype = Size109Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<109,1> {}
                    
/// A type with a size of `110` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size110Align1([::core::mem::MaybeUninit<u8>; 110]);
impl Size110Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<110,1> { type Archetype = Size110Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<110,1> {}
                    
/// A type with a size of `110` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size110Align2([::core::mem::MaybeUninit<u16>; 55]);
impl Size110Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<110,2> { type Archetype = Size110Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<110,2> {}
                    
/// A type with a size of `111` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size111Align1([::core::mem::MaybeUninit<u8>; 111]);
impl Size111Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<111,1> { type Archetype = Size111Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<111,1> {}
                    
/// A type with a size of `112` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size112Align1([::core::mem::MaybeUninit<u8>; 112]);
impl Size112Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<112,1> { type Archetype = Size112Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<112,1> {}
                    
/// A type with a size of `112` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size112Align2([::core::mem::MaybeUninit<u16>; 56]);
impl Size112Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<112,2> { type Archetype = Size112Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<112,2> {}
                    
/// A type with a size of `112` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size112Align4([::core::mem::MaybeUninit<u32>; 28]);
impl Size112Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<112,4> { type Archetype = Size112Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<112,4> {}
                    
/// A type with a size of `112` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size112Align8([::core::mem::MaybeUninit<u64>; 14]);
impl Size112Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<112,8> { type Archetype = Size112Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<112,8> {}
                    
/// A type with a size of `112` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size112Align16([::core::mem::MaybeUninit<u128>; 7]);
impl Size112Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<112,16> { type Archetype = Size112Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<112,16> {}
                    
/// A type with a size of `113` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size113Align1([::core::mem::MaybeUninit<u8>; 113]);
impl Size113Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<113,1> { type Archetype = Size113Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<113,1> {}
                    
/// A type with a size of `114` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size114Align1([::core::mem::MaybeUninit<u8>; 114]);
impl Size114Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<114,1> { type Archetype = Size114Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<114,1> {}
                    
/// A type with a size of `114` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size114Align2([::core::mem::MaybeUninit<u16>; 57]);
impl Size114Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<114,2> { type Archetype = Size114Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<114,2> {}
                    
/// A type with a size of `115` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size115Align1([::core::mem::MaybeUninit<u8>; 115]);
impl Size115Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<115,1> { type Archetype = Size115Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<115,1> {}
                    
/// A type with a size of `116` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size116Align1([::core::mem::MaybeUninit<u8>; 116]);
impl Size116Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<116,1> { type Archetype = Size116Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<116,1> {}
                    
/// A type with a size of `116` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size116Align2([::core::mem::MaybeUninit<u16>; 58]);
impl Size116Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<116,2> { type Archetype = Size116Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<116,2> {}
                    
/// A type with a size of `116` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size116Align4([::core::mem::MaybeUninit<u32>; 29]);
impl Size116Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<116,4> { type Archetype = Size116Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<116,4> {}
                    
/// A type with a size of `117` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size117Align1([::core::mem::MaybeUninit<u8>; 117]);
impl Size117Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<117,1> { type Archetype = Size117Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<117,1> {}
                    
/// A type with a size of `118` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size118Align1([::core::mem::MaybeUninit<u8>; 118]);
impl Size118Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<118,1> { type Archetype = Size118Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<118,1> {}
                    
/// A type with a size of `118` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size118Align2([::core::mem::MaybeUninit<u16>; 59]);
impl Size118Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<118,2> { type Archetype = Size118Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<118,2> {}
                    
/// A type with a size of `119` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size119Align1([::core::mem::MaybeUninit<u8>; 119]);
impl Size119Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<119,1> { type Archetype = Size119Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<119,1> {}
                    
/// A type with a size of `120` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size120Align1([::core::mem::MaybeUninit<u8>; 120]);
impl Size120Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<120,1> { type Archetype = Size120Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<120,1> {}
                    
/// A type with a size of `120` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size120Align2([::core::mem::MaybeUninit<u16>; 60]);
impl Size120Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<120,2> { type Archetype = Size120Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<120,2> {}
                    
/// A type with a size of `120` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size120Align4([::core::mem::MaybeUninit<u32>; 30]);
impl Size120Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<120,4> { type Archetype = Size120Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<120,4> {}
                    
/// A type with a size of `120` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size120Align8([::core::mem::MaybeUninit<u64>; 15]);
impl Size120Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<120,8> { type Archetype = Size120Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<120,8> {}
                    
/// A type with a size of `121` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size121Align1([::core::mem::MaybeUninit<u8>; 121]);
impl Size121Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<121,1> { type Archetype = Size121Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<121,1> {}
                    
/// A type with a size of `122` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size122Align1([::core::mem::MaybeUninit<u8>; 122]);
impl Size122Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<122,1> { type Archetype = Size122Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<122,1> {}
                    
/// A type with a size of `122` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size122Align2([::core::mem::MaybeUninit<u16>; 61]);
impl Size122Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<122,2> { type Archetype = Size122Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<122,2> {}
                    
/// A type with a size of `123` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size123Align1([::core::mem::MaybeUninit<u8>; 123]);
impl Size123Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<123,1> { type Archetype = Size123Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<123,1> {}
                    
/// A type with a size of `124` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size124Align1([::core::mem::MaybeUninit<u8>; 124]);
impl Size124Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<124,1> { type Archetype = Size124Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<124,1> {}
                    
/// A type with a size of `124` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size124Align2([::core::mem::MaybeUninit<u16>; 62]);
impl Size124Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<124,2> { type Archetype = Size124Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<124,2> {}
                    
/// A type with a size of `124` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size124Align4([::core::mem::MaybeUninit<u32>; 31]);
impl Size124Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<124,4> { type Archetype = Size124Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<124,4> {}
                    
/// A type with a size of `125` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size125Align1([::core::mem::MaybeUninit<u8>; 125]);
impl Size125Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<125,1> { type Archetype = Size125Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<125,1> {}
                    
/// A type with a size of `126` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size126Align1([::core::mem::MaybeUninit<u8>; 126]);
impl Size126Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<126,1> { type Archetype = Size126Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<126,1> {}
                    
/// A type with a size of `126` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size126Align2([::core::mem::MaybeUninit<u16>; 63]);
impl Size126Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<126,2> { type Archetype = Size126Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<126,2> {}
                    
/// A type with a size of `127` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size127Align1([::core::mem::MaybeUninit<u8>; 127]);
impl Size127Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<127,1> { type Archetype = Size127Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<127,1> {}
                    
/// A type with a size of `128` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size128Align1([::core::mem::MaybeUninit<u8>; 128]);
impl Size128Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<128,1> { type Archetype = Size128Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<128,1> {}
                    
/// A type with a size of `128` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size128Align2([::core::mem::MaybeUninit<u16>; 64]);
impl Size128Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<128,2> { type Archetype = Size128Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<128,2> {}
                    
/// A type with a size of `128` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size128Align4([::core::mem::MaybeUninit<u32>; 32]);
impl Size128Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<128,4> { type Archetype = Size128Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<128,4> {}
                    
/// A type with a size of `128` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size128Align8([::core::mem::MaybeUninit<u64>; 16]);
impl Size128Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<128,8> { type Archetype = Size128Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<128,8> {}
                    
/// A type with a size of `128` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size128Align16([::core::mem::MaybeUninit<u128>; 8]);
impl Size128Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<128,16> { type Archetype = Size128Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<128,16> {}
                    
/// A type with a size of `129` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size129Align1([::core::mem::MaybeUninit<u8>; 129]);
impl Size129Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<129,1> { type Archetype = Size129Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<129,1> {}
                    
/// A type with a size of `130` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size130Align1([::core::mem::MaybeUninit<u8>; 130]);
impl Size130Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<130,1> { type Archetype = Size130Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<130,1> {}
                    
/// A type with a size of `130` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size130Align2([::core::mem::MaybeUninit<u16>; 65]);
impl Size130Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<130,2> { type Archetype = Size130Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<130,2> {}
                    
/// A type with a size of `131` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size131Align1([::core::mem::MaybeUninit<u8>; 131]);
impl Size131Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<131,1> { type Archetype = Size131Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<131,1> {}
                    
/// A type with a size of `132` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size132Align1([::core::mem::MaybeUninit<u8>; 132]);
impl Size132Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<132,1> { type Archetype = Size132Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<132,1> {}
                    
/// A type with a size of `132` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size132Align2([::core::mem::MaybeUninit<u16>; 66]);
impl Size132Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<132,2> { type Archetype = Size132Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<132,2> {}
                    
/// A type with a size of `132` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size132Align4([::core::mem::MaybeUninit<u32>; 33]);
impl Size132Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<132,4> { type Archetype = Size132Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<132,4> {}
                    
/// A type with a size of `133` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size133Align1([::core::mem::MaybeUninit<u8>; 133]);
impl Size133Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<133,1> { type Archetype = Size133Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<133,1> {}
                    
/// A type with a size of `134` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size134Align1([::core::mem::MaybeUninit<u8>; 134]);
impl Size134Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<134,1> { type Archetype = Size134Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<134,1> {}
                    
/// A type with a size of `134` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size134Align2([::core::mem::MaybeUninit<u16>; 67]);
impl Size134Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<134,2> { type Archetype = Size134Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<134,2> {}
                    
/// A type with a size of `135` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size135Align1([::core::mem::MaybeUninit<u8>; 135]);
impl Size135Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<135,1> { type Archetype = Size135Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<135,1> {}
                    
/// A type with a size of `136` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size136Align1([::core::mem::MaybeUninit<u8>; 136]);
impl Size136Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<136,1> { type Archetype = Size136Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<136,1> {}
                    
/// A type with a size of `136` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size136Align2([::core::mem::MaybeUninit<u16>; 68]);
impl Size136Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<136,2> { type Archetype = Size136Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<136,2> {}
                    
/// A type with a size of `136` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size136Align4([::core::mem::MaybeUninit<u32>; 34]);
impl Size136Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<136,4> { type Archetype = Size136Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<136,4> {}
                    
/// A type with a size of `136` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size136Align8([::core::mem::MaybeUninit<u64>; 17]);
impl Size136Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<136,8> { type Archetype = Size136Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<136,8> {}
                    
/// A type with a size of `137` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size137Align1([::core::mem::MaybeUninit<u8>; 137]);
impl Size137Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<137,1> { type Archetype = Size137Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<137,1> {}
                    
/// A type with a size of `138` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size138Align1([::core::mem::MaybeUninit<u8>; 138]);
impl Size138Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<138,1> { type Archetype = Size138Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<138,1> {}
                    
/// A type with a size of `138` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size138Align2([::core::mem::MaybeUninit<u16>; 69]);
impl Size138Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<138,2> { type Archetype = Size138Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<138,2> {}
                    
/// A type with a size of `139` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size139Align1([::core::mem::MaybeUninit<u8>; 139]);
impl Size139Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<139,1> { type Archetype = Size139Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<139,1> {}
                    
/// A type with a size of `140` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size140Align1([::core::mem::MaybeUninit<u8>; 140]);
impl Size140Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<140,1> { type Archetype = Size140Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<140,1> {}
                    
/// A type with a size of `140` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size140Align2([::core::mem::MaybeUninit<u16>; 70]);
impl Size140Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<140,2> { type Archetype = Size140Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<140,2> {}
                    
/// A type with a size of `140` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size140Align4([::core::mem::MaybeUninit<u32>; 35]);
impl Size140Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<140,4> { type Archetype = Size140Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<140,4> {}
                    
/// A type with a size of `141` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size141Align1([::core::mem::MaybeUninit<u8>; 141]);
impl Size141Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<141,1> { type Archetype = Size141Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<141,1> {}
                    
/// A type with a size of `142` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size142Align1([::core::mem::MaybeUninit<u8>; 142]);
impl Size142Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<142,1> { type Archetype = Size142Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<142,1> {}
                    
/// A type with a size of `142` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size142Align2([::core::mem::MaybeUninit<u16>; 71]);
impl Size142Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<142,2> { type Archetype = Size142Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<142,2> {}
                    
/// A type with a size of `143` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size143Align1([::core::mem::MaybeUninit<u8>; 143]);
impl Size143Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<143,1> { type Archetype = Size143Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<143,1> {}
                    
/// A type with a size of `144` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size144Align1([::core::mem::MaybeUninit<u8>; 144]);
impl Size144Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<144,1> { type Archetype = Size144Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<144,1> {}
                    
/// A type with a size of `144` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size144Align2([::core::mem::MaybeUninit<u16>; 72]);
impl Size144Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<144,2> { type Archetype = Size144Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<144,2> {}
                    
/// A type with a size of `144` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size144Align4([::core::mem::MaybeUninit<u32>; 36]);
impl Size144Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<144,4> { type Archetype = Size144Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<144,4> {}
                    
/// A type with a size of `144` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size144Align8([::core::mem::MaybeUninit<u64>; 18]);
impl Size144Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<144,8> { type Archetype = Size144Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<144,8> {}
                    
/// A type with a size of `144` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size144Align16([::core::mem::MaybeUninit<u128>; 9]);
impl Size144Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<144,16> { type Archetype = Size144Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<144,16> {}
                    
/// A type with a size of `145` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size145Align1([::core::mem::MaybeUninit<u8>; 145]);
impl Size145Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<145,1> { type Archetype = Size145Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<145,1> {}
                    
/// A type with a size of `146` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size146Align1([::core::mem::MaybeUninit<u8>; 146]);
impl Size146Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<146,1> { type Archetype = Size146Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<146,1> {}
                    
/// A type with a size of `146` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size146Align2([::core::mem::MaybeUninit<u16>; 73]);
impl Size146Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<146,2> { type Archetype = Size146Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<146,2> {}
                    
/// A type with a size of `147` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size147Align1([::core::mem::MaybeUninit<u8>; 147]);
impl Size147Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<147,1> { type Archetype = Size147Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<147,1> {}
                    
/// A type with a size of `148` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size148Align1([::core::mem::MaybeUninit<u8>; 148]);
impl Size148Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<148,1> { type Archetype = Size148Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<148,1> {}
                    
/// A type with a size of `148` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size148Align2([::core::mem::MaybeUninit<u16>; 74]);
impl Size148Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<148,2> { type Archetype = Size148Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<148,2> {}
                    
/// A type with a size of `148` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size148Align4([::core::mem::MaybeUninit<u32>; 37]);
impl Size148Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<148,4> { type Archetype = Size148Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<148,4> {}
                    
/// A type with a size of `149` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size149Align1([::core::mem::MaybeUninit<u8>; 149]);
impl Size149Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<149,1> { type Archetype = Size149Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<149,1> {}
                    
/// A type with a size of `150` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size150Align1([::core::mem::MaybeUninit<u8>; 150]);
impl Size150Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<150,1> { type Archetype = Size150Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<150,1> {}
                    
/// A type with a size of `150` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size150Align2([::core::mem::MaybeUninit<u16>; 75]);
impl Size150Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<150,2> { type Archetype = Size150Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<150,2> {}
                    
/// A type with a size of `151` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size151Align1([::core::mem::MaybeUninit<u8>; 151]);
impl Size151Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<151,1> { type Archetype = Size151Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<151,1> {}
                    
/// A type with a size of `152` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size152Align1([::core::mem::MaybeUninit<u8>; 152]);
impl Size152Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<152,1> { type Archetype = Size152Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<152,1> {}
                    
/// A type with a size of `152` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size152Align2([::core::mem::MaybeUninit<u16>; 76]);
impl Size152Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<152,2> { type Archetype = Size152Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<152,2> {}
                    
/// A type with a size of `152` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size152Align4([::core::mem::MaybeUninit<u32>; 38]);
impl Size152Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<152,4> { type Archetype = Size152Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<152,4> {}
                    
/// A type with a size of `152` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size152Align8([::core::mem::MaybeUninit<u64>; 19]);
impl Size152Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<152,8> { type Archetype = Size152Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<152,8> {}
                    
/// A type with a size of `153` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size153Align1([::core::mem::MaybeUninit<u8>; 153]);
impl Size153Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<153,1> { type Archetype = Size153Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<153,1> {}
                    
/// A type with a size of `154` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size154Align1([::core::mem::MaybeUninit<u8>; 154]);
impl Size154Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<154,1> { type Archetype = Size154Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<154,1> {}
                    
/// A type with a size of `154` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size154Align2([::core::mem::MaybeUninit<u16>; 77]);
impl Size154Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<154,2> { type Archetype = Size154Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<154,2> {}
                    
/// A type with a size of `155` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size155Align1([::core::mem::MaybeUninit<u8>; 155]);
impl Size155Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<155,1> { type Archetype = Size155Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<155,1> {}
                    
/// A type with a size of `156` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size156Align1([::core::mem::MaybeUninit<u8>; 156]);
impl Size156Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<156,1> { type Archetype = Size156Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<156,1> {}
                    
/// A type with a size of `156` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size156Align2([::core::mem::MaybeUninit<u16>; 78]);
impl Size156Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<156,2> { type Archetype = Size156Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<156,2> {}
                    
/// A type with a size of `156` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size156Align4([::core::mem::MaybeUninit<u32>; 39]);
impl Size156Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<156,4> { type Archetype = Size156Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<156,4> {}
                    
/// A type with a size of `157` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size157Align1([::core::mem::MaybeUninit<u8>; 157]);
impl Size157Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<157,1> { type Archetype = Size157Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<157,1> {}
                    
/// A type with a size of `158` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size158Align1([::core::mem::MaybeUninit<u8>; 158]);
impl Size158Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<158,1> { type Archetype = Size158Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<158,1> {}
                    
/// A type with a size of `158` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size158Align2([::core::mem::MaybeUninit<u16>; 79]);
impl Size158Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<158,2> { type Archetype = Size158Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<158,2> {}
                    
/// A type with a size of `159` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size159Align1([::core::mem::MaybeUninit<u8>; 159]);
impl Size159Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<159,1> { type Archetype = Size159Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<159,1> {}
                    
/// A type with a size of `160` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size160Align1([::core::mem::MaybeUninit<u8>; 160]);
impl Size160Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<160,1> { type Archetype = Size160Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<160,1> {}
                    
/// A type with a size of `160` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size160Align2([::core::mem::MaybeUninit<u16>; 80]);
impl Size160Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<160,2> { type Archetype = Size160Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<160,2> {}
                    
/// A type with a size of `160` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size160Align4([::core::mem::MaybeUninit<u32>; 40]);
impl Size160Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<160,4> { type Archetype = Size160Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<160,4> {}
                    
/// A type with a size of `160` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size160Align8([::core::mem::MaybeUninit<u64>; 20]);
impl Size160Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<160,8> { type Archetype = Size160Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<160,8> {}
                    
/// A type with a size of `160` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size160Align16([::core::mem::MaybeUninit<u128>; 10]);
impl Size160Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<160,16> { type Archetype = Size160Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<160,16> {}
                    
/// A type with a size of `161` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size161Align1([::core::mem::MaybeUninit<u8>; 161]);
impl Size161Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<161,1> { type Archetype = Size161Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<161,1> {}
                    
/// A type with a size of `162` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size162Align1([::core::mem::MaybeUninit<u8>; 162]);
impl Size162Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<162,1> { type Archetype = Size162Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<162,1> {}
                    
/// A type with a size of `162` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size162Align2([::core::mem::MaybeUninit<u16>; 81]);
impl Size162Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<162,2> { type Archetype = Size162Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<162,2> {}
                    
/// A type with a size of `163` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size163Align1([::core::mem::MaybeUninit<u8>; 163]);
impl Size163Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<163,1> { type Archetype = Size163Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<163,1> {}
                    
/// A type with a size of `164` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size164Align1([::core::mem::MaybeUninit<u8>; 164]);
impl Size164Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<164,1> { type Archetype = Size164Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<164,1> {}
                    
/// A type with a size of `164` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size164Align2([::core::mem::MaybeUninit<u16>; 82]);
impl Size164Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<164,2> { type Archetype = Size164Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<164,2> {}
                    
/// A type with a size of `164` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size164Align4([::core::mem::MaybeUninit<u32>; 41]);
impl Size164Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<164,4> { type Archetype = Size164Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<164,4> {}
                    
/// A type with a size of `165` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size165Align1([::core::mem::MaybeUninit<u8>; 165]);
impl Size165Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<165,1> { type Archetype = Size165Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<165,1> {}
                    
/// A type with a size of `166` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size166Align1([::core::mem::MaybeUninit<u8>; 166]);
impl Size166Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<166,1> { type Archetype = Size166Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<166,1> {}
                    
/// A type with a size of `166` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size166Align2([::core::mem::MaybeUninit<u16>; 83]);
impl Size166Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<166,2> { type Archetype = Size166Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<166,2> {}
                    
/// A type with a size of `167` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size167Align1([::core::mem::MaybeUninit<u8>; 167]);
impl Size167Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<167,1> { type Archetype = Size167Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<167,1> {}
                    
/// A type with a size of `168` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size168Align1([::core::mem::MaybeUninit<u8>; 168]);
impl Size168Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<168,1> { type Archetype = Size168Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<168,1> {}
                    
/// A type with a size of `168` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size168Align2([::core::mem::MaybeUninit<u16>; 84]);
impl Size168Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<168,2> { type Archetype = Size168Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<168,2> {}
                    
/// A type with a size of `168` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size168Align4([::core::mem::MaybeUninit<u32>; 42]);
impl Size168Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<168,4> { type Archetype = Size168Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<168,4> {}
                    
/// A type with a size of `168` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size168Align8([::core::mem::MaybeUninit<u64>; 21]);
impl Size168Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<168,8> { type Archetype = Size168Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<168,8> {}
                    
/// A type with a size of `169` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size169Align1([::core::mem::MaybeUninit<u8>; 169]);
impl Size169Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<169,1> { type Archetype = Size169Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<169,1> {}
                    
/// A type with a size of `170` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size170Align1([::core::mem::MaybeUninit<u8>; 170]);
impl Size170Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<170,1> { type Archetype = Size170Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<170,1> {}
                    
/// A type with a size of `170` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size170Align2([::core::mem::MaybeUninit<u16>; 85]);
impl Size170Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<170,2> { type Archetype = Size170Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<170,2> {}
                    
/// A type with a size of `171` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size171Align1([::core::mem::MaybeUninit<u8>; 171]);
impl Size171Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<171,1> { type Archetype = Size171Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<171,1> {}
                    
/// A type with a size of `172` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size172Align1([::core::mem::MaybeUninit<u8>; 172]);
impl Size172Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<172,1> { type Archetype = Size172Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<172,1> {}
                    
/// A type with a size of `172` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size172Align2([::core::mem::MaybeUninit<u16>; 86]);
impl Size172Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<172,2> { type Archetype = Size172Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<172,2> {}
                    
/// A type with a size of `172` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size172Align4([::core::mem::MaybeUninit<u32>; 43]);
impl Size172Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<172,4> { type Archetype = Size172Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<172,4> {}
                    
/// A type with a size of `173` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size173Align1([::core::mem::MaybeUninit<u8>; 173]);
impl Size173Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<173,1> { type Archetype = Size173Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<173,1> {}
                    
/// A type with a size of `174` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size174Align1([::core::mem::MaybeUninit<u8>; 174]);
impl Size174Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<174,1> { type Archetype = Size174Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<174,1> {}
                    
/// A type with a size of `174` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size174Align2([::core::mem::MaybeUninit<u16>; 87]);
impl Size174Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<174,2> { type Archetype = Size174Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<174,2> {}
                    
/// A type with a size of `175` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size175Align1([::core::mem::MaybeUninit<u8>; 175]);
impl Size175Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<175,1> { type Archetype = Size175Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<175,1> {}
                    
/// A type with a size of `176` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size176Align1([::core::mem::MaybeUninit<u8>; 176]);
impl Size176Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<176,1> { type Archetype = Size176Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<176,1> {}
                    
/// A type with a size of `176` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size176Align2([::core::mem::MaybeUninit<u16>; 88]);
impl Size176Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<176,2> { type Archetype = Size176Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<176,2> {}
                    
/// A type with a size of `176` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size176Align4([::core::mem::MaybeUninit<u32>; 44]);
impl Size176Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<176,4> { type Archetype = Size176Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<176,4> {}
                    
/// A type with a size of `176` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size176Align8([::core::mem::MaybeUninit<u64>; 22]);
impl Size176Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<176,8> { type Archetype = Size176Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<176,8> {}
                    
/// A type with a size of `176` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size176Align16([::core::mem::MaybeUninit<u128>; 11]);
impl Size176Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<176,16> { type Archetype = Size176Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<176,16> {}
                    
/// A type with a size of `177` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size177Align1([::core::mem::MaybeUninit<u8>; 177]);
impl Size177Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<177,1> { type Archetype = Size177Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<177,1> {}
                    
/// A type with a size of `178` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size178Align1([::core::mem::MaybeUninit<u8>; 178]);
impl Size178Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<178,1> { type Archetype = Size178Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<178,1> {}
                    
/// A type with a size of `178` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size178Align2([::core::mem::MaybeUninit<u16>; 89]);
impl Size178Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<178,2> { type Archetype = Size178Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<178,2> {}
                    
/// A type with a size of `179` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size179Align1([::core::mem::MaybeUninit<u8>; 179]);
impl Size179Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<179,1> { type Archetype = Size179Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<179,1> {}
                    
/// A type with a size of `180` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size180Align1([::core::mem::MaybeUninit<u8>; 180]);
impl Size180Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<180,1> { type Archetype = Size180Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<180,1> {}
                    
/// A type with a size of `180` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size180Align2([::core::mem::MaybeUninit<u16>; 90]);
impl Size180Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<180,2> { type Archetype = Size180Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<180,2> {}
                    
/// A type with a size of `180` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size180Align4([::core::mem::MaybeUninit<u32>; 45]);
impl Size180Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<180,4> { type Archetype = Size180Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<180,4> {}
                    
/// A type with a size of `181` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size181Align1([::core::mem::MaybeUninit<u8>; 181]);
impl Size181Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<181,1> { type Archetype = Size181Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<181,1> {}
                    
/// A type with a size of `182` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size182Align1([::core::mem::MaybeUninit<u8>; 182]);
impl Size182Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<182,1> { type Archetype = Size182Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<182,1> {}
                    
/// A type with a size of `182` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size182Align2([::core::mem::MaybeUninit<u16>; 91]);
impl Size182Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<182,2> { type Archetype = Size182Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<182,2> {}
                    
/// A type with a size of `183` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size183Align1([::core::mem::MaybeUninit<u8>; 183]);
impl Size183Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<183,1> { type Archetype = Size183Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<183,1> {}
                    
/// A type with a size of `184` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size184Align1([::core::mem::MaybeUninit<u8>; 184]);
impl Size184Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<184,1> { type Archetype = Size184Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<184,1> {}
                    
/// A type with a size of `184` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size184Align2([::core::mem::MaybeUninit<u16>; 92]);
impl Size184Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<184,2> { type Archetype = Size184Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<184,2> {}
                    
/// A type with a size of `184` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size184Align4([::core::mem::MaybeUninit<u32>; 46]);
impl Size184Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<184,4> { type Archetype = Size184Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<184,4> {}
                    
/// A type with a size of `184` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size184Align8([::core::mem::MaybeUninit<u64>; 23]);
impl Size184Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<184,8> { type Archetype = Size184Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<184,8> {}
                    
/// A type with a size of `185` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size185Align1([::core::mem::MaybeUninit<u8>; 185]);
impl Size185Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<185,1> { type Archetype = Size185Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<185,1> {}
                    
/// A type with a size of `186` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size186Align1([::core::mem::MaybeUninit<u8>; 186]);
impl Size186Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<186,1> { type Archetype = Size186Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<186,1> {}
                    
/// A type with a size of `186` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size186Align2([::core::mem::MaybeUninit<u16>; 93]);
impl Size186Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<186,2> { type Archetype = Size186Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<186,2> {}
                    
/// A type with a size of `187` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size187Align1([::core::mem::MaybeUninit<u8>; 187]);
impl Size187Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<187,1> { type Archetype = Size187Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<187,1> {}
                    
/// A type with a size of `188` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size188Align1([::core::mem::MaybeUninit<u8>; 188]);
impl Size188Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<188,1> { type Archetype = Size188Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<188,1> {}
                    
/// A type with a size of `188` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size188Align2([::core::mem::MaybeUninit<u16>; 94]);
impl Size188Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<188,2> { type Archetype = Size188Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<188,2> {}
                    
/// A type with a size of `188` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size188Align4([::core::mem::MaybeUninit<u32>; 47]);
impl Size188Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<188,4> { type Archetype = Size188Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<188,4> {}
                    
/// A type with a size of `189` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size189Align1([::core::mem::MaybeUninit<u8>; 189]);
impl Size189Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<189,1> { type Archetype = Size189Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<189,1> {}
                    
/// A type with a size of `190` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size190Align1([::core::mem::MaybeUninit<u8>; 190]);
impl Size190Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<190,1> { type Archetype = Size190Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<190,1> {}
                    
/// A type with a size of `190` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size190Align2([::core::mem::MaybeUninit<u16>; 95]);
impl Size190Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<190,2> { type Archetype = Size190Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<190,2> {}
                    
/// A type with a size of `191` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size191Align1([::core::mem::MaybeUninit<u8>; 191]);
impl Size191Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<191,1> { type Archetype = Size191Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<191,1> {}
                    
/// A type with a size of `192` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size192Align1([::core::mem::MaybeUninit<u8>; 192]);
impl Size192Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<192,1> { type Archetype = Size192Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<192,1> {}
                    
/// A type with a size of `192` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size192Align2([::core::mem::MaybeUninit<u16>; 96]);
impl Size192Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<192,2> { type Archetype = Size192Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<192,2> {}
                    
/// A type with a size of `192` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size192Align4([::core::mem::MaybeUninit<u32>; 48]);
impl Size192Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<192,4> { type Archetype = Size192Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<192,4> {}
                    
/// A type with a size of `192` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size192Align8([::core::mem::MaybeUninit<u64>; 24]);
impl Size192Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<192,8> { type Archetype = Size192Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<192,8> {}
                    
/// A type with a size of `192` bytes and alignment `16`.
#[repr(C, align(16))]
pub struct Size192Align16([::core::mem::MaybeUninit<u128>; 12]);
impl Size192Align16 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<192,16> { type Archetype = Size192Align16; }
impl crate::private::Sealed for crate::SizeAndAlign<192,16> {}
                    
/// A type with a size of `193` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size193Align1([::core::mem::MaybeUninit<u8>; 193]);
impl Size193Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<193,1> { type Archetype = Size193Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<193,1> {}
                    
/// A type with a size of `194` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size194Align1([::core::mem::MaybeUninit<u8>; 194]);
impl Size194Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<194,1> { type Archetype = Size194Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<194,1> {}
                    
/// A type with a size of `194` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size194Align2([::core::mem::MaybeUninit<u16>; 97]);
impl Size194Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<194,2> { type Archetype = Size194Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<194,2> {}
                    
/// A type with a size of `195` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size195Align1([::core::mem::MaybeUninit<u8>; 195]);
impl Size195Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<195,1> { type Archetype = Size195Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<195,1> {}
                    
/// A type with a size of `196` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size196Align1([::core::mem::MaybeUninit<u8>; 196]);
impl Size196Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<196,1> { type Archetype = Size196Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<196,1> {}
                    
/// A type with a size of `196` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size196Align2([::core::mem::MaybeUninit<u16>; 98]);
impl Size196Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<196,2> { type Archetype = Size196Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<196,2> {}
                    
/// A type with a size of `196` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size196Align4([::core::mem::MaybeUninit<u32>; 49]);
impl Size196Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<196,4> { type Archetype = Size196Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<196,4> {}
                    
/// A type with a size of `197` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size197Align1([::core::mem::MaybeUninit<u8>; 197]);
impl Size197Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<197,1> { type Archetype = Size197Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<197,1> {}
                    
/// A type with a size of `198` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size198Align1([::core::mem::MaybeUninit<u8>; 198]);
impl Size198Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<198,1> { type Archetype = Size198Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<198,1> {}
                    
/// A type with a size of `198` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size198Align2([::core::mem::MaybeUninit<u16>; 99]);
impl Size198Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<198,2> { type Archetype = Size198Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<198,2> {}
                    
/// A type with a size of `199` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size199Align1([::core::mem::MaybeUninit<u8>; 199]);
impl Size199Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<199,1> { type Archetype = Size199Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<199,1> {}
                    
/// A type with a size of `200` bytes and alignment `1`.
#[repr(C, align(1))]
pub struct Size200Align1([::core::mem::MaybeUninit<u8>; 200]);
impl Size200Align1 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<200,1> { type Archetype = Size200Align1; }
impl crate::private::Sealed for crate::SizeAndAlign<200,1> {}
                    
/// A type with a size of `200` bytes and alignment `2`.
#[repr(C, align(2))]
pub struct Size200Align2([::core::mem::MaybeUninit<u16>; 100]);
impl Size200Align2 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<200,2> { type Archetype = Size200Align2; }
impl crate::private::Sealed for crate::SizeAndAlign<200,2> {}
                    
/// A type with a size of `200` bytes and alignment `4`.
#[repr(C, align(4))]
pub struct Size200Align4([::core::mem::MaybeUninit<u32>; 50]);
impl Size200Align4 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<200,4> { type Archetype = Size200Align4; }
impl crate::private::Sealed for crate::SizeAndAlign<200,4> {}
                    
/// A type with a size of `200` bytes and alignment `8`.
#[repr(C, align(8))]
pub struct Size200Align8([::core::mem::MaybeUninit<u64>; 25]);
impl Size200Align8 {
    pub const unsafe fn new_uninit() -> Self {
        Self([::core::mem::MaybeUninit::uninit(); _])
    }
    pub const unsafe fn new_zeroed() -> Self {
        Self([::core::mem::MaybeUninit::zeroed(); _])
    }
}

unsafe impl crate::Mimic for crate::SizeAndAlign<200,8> { type Archetype = Size200Align8; }
impl crate::private::Sealed for crate::SizeAndAlign<200,8> {}
                    