use crate::models::{GPR, Immediate8, Immediate32};

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

/// Represents the offset for a memory operand.
///
/// # Values
///
/// - `None` - No offset.
/// - `Imm8` - 8-bit signed immediate offset.
/// - `Imm32` - 32-bit signed immediate offset.
///
/// # Notes
///
/// The offset is always understood as signed integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum Offset {
    None,
    Bit8(Immediate8),
    Bit32(Immediate32),
}

impl Offset {
    #[inline(always)]
    pub const fn from_i8(value: i8) -> Self {
        Self::Bit8(Immediate8::from_i8(value))
    }

    #[inline(always)]
    pub const fn from_i32(value: i32) -> Self {
        Self::Bit32(Immediate32::from_i32(value))
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Represents a memory operand.
///
/// # Safety
///
/// Not all combinations of are valid for a given instruction.
/// For specifics refer to the Intel x86 manual.
///
/// In particular all the GPRs have to be 64-bit wide.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub enum Memory {
    Based {
        base: GPR,
        offset: Offset,
    },
    Scaled {
        index: GPR,
        scale: Scale,
        offset: Offset,
    },
    BasedScaled {
        base: GPR,
        index: GPR,
        scale: Scale,
        offset: Offset,
    },
    RelativeToRIP {
        offset: Offset,
    },
}

impl Memory {
    #[inline(always)]
    #[must_use]
    pub const fn base_is_extended(&self) -> bool {
        match self {
            Self::Based { base, .. } => base.is_extended(),
            Self::BasedScaled { base, .. } => base.is_extended(),
            _ => false,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn index_is_extended(&self) -> bool {
        match self {
            Self::Scaled { index, .. } => index.is_extended(),
            Self::BasedScaled { index, .. } => index.is_extended(),
            _ => false,
        }
    }
}
