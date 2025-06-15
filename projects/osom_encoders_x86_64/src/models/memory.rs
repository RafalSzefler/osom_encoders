use osom_encoders_common::osom_debug_assert;

use crate::models::{GPR, Immediate8, Immediate32, Size};

/// Represents the scale factor for the index register in a memory operand.
///
/// # Values
///
/// - `Scale1` - The index register is multiplied by 1.
/// - `Scale2` - The index register is multiplied by 2.
/// - `Scale4` - The index register is multiplied by 4.
/// - `Scale8` - The index register is multiplied by 8.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum Scale {
    Scale1 = 1, // We start from 1 to allow Option<Scale> optimization
    Scale2,
    Scale4,
    Scale8,
}

/// Represents the displacement for a memory operand.
///
/// # Values
///
/// - `None` - No displacement.
/// - `Imm8` - 8-bit immediate displacement.
/// - `Imm16` - 16-bit immediate displacement.
/// - `Imm32` - 32-bit immediate displacement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum Displacement {
    None,
    Imm8(Immediate8),
    Imm32(Immediate32),
}

impl Displacement {
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        Self::Imm8(Immediate8::from_u8(value))
    }

    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self::Imm32(Immediate32::from_u32(value))
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Represents a memory operand.
///
/// # Values
///
/// - `base` - The base register, 64-bit wide.
/// - `index` - The index register, 64-bit wide.
/// - `scale` - The scale factor for the index register.
/// - `displacement` - The displacement for the memory operand.
///
/// # Safety
///
/// Not all combinations of `base`, `index`, `scale`, and `displacement` are valid
/// for a given instruction. For specifics refer to the Intel x86 manual.
///
/// In particular the GPRs have to be 64-bit wide.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct Memory {
    base: Option<GPR>,
    index: Option<GPR>,
    scale: Option<Scale>,
    displacement: Displacement,
}

impl Memory {
    /// Creates a new memory operand.
    ///
    /// # Safety
    ///
    /// The caller *must* ensure that the memory operand is valid for the given instruction.
    /// Also the encoder supports 64-bit memory addressing only. In particular all the GPRs
    /// have to be 64-bit wide.
    #[inline(always)]
    pub const unsafe fn new(
        base: Option<GPR>,
        index: Option<GPR>,
        scale: Option<Scale>,
        displacement: Displacement,
    ) -> Self {
        osom_debug_assert!(base.is_none() || base.unwrap().size().equals(Size::Bit64));
        osom_debug_assert!(index.is_none() || index.unwrap().size().equals(Size::Bit64));
        osom_debug_assert!(base.is_some() || index.is_some());
        osom_debug_assert!(index.is_none() || scale.is_some());

        Self {
            base,
            index,
            scale,
            displacement,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn base(&self) -> Option<GPR> {
        unsafe { core::hint::assert_unchecked(self.base.is_none() || self.base.unwrap().size().equals(Size::Bit64)) };
        self.base
    }

    #[inline(always)]
    #[must_use]
    pub const fn index(&self) -> Option<GPR> {
        unsafe { core::hint::assert_unchecked(self.index.is_none() || self.index.unwrap().size().equals(Size::Bit64)) };
        self.index
    }

    #[inline(always)]
    #[must_use]
    pub const fn scale(&self) -> Option<Scale> {
        self.scale
    }

    #[inline(always)]
    pub const fn displacement(&self) -> &Displacement {
        &self.displacement
    }
}
