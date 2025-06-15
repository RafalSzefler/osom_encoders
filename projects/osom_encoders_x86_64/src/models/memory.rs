use crate::models::{GPR, Immediate8, Immediate16, Immediate32};

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
    Imm16(Immediate16),
    Imm32(Immediate32),
}

impl Displacement {
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        Self::Imm8(Immediate8::from_u8(value))
    }

    #[inline(always)]
    pub const fn from_u16(value: u16) -> Self {
        Self::Imm16(Immediate16::from_u16(value))
    }

    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self::Imm32(Immediate32::from_u32(value))
    }
}

/// Represents a memory operand.
///
/// # Values
///
/// - `base` - The base register.
/// - `index` - The index register.
/// - `scale` - The scale factor for the index register.
/// - `displacement` - The displacement for the memory operand.
///
/// # Safety
///
/// Not all combinations of `base`, `index`, `scale`, and `displacement` are valid
/// for a given instruction. For specifics refer to the Intel x86 manual.
///
/// In particular the GPRs have to be 32 or 64 bit wide, depending on the architecture.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct Memory {
    base: Option<GPR>,
    index: Option<GPR>,
    scale: Option<Scale>,
    displacement: Displacement,
}

impl Memory {
    #[inline(always)]
    pub const fn new(base: Option<GPR>, index: Option<GPR>, scale: Option<Scale>, displacement: Displacement) -> Self {
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
        self.base
    }

    #[inline(always)]
    #[must_use]
    pub const fn index(&self) -> Option<GPR> {
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
