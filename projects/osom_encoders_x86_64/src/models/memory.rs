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

impl Scale {
    #[inline(always)]
    pub(crate) const fn index(self) -> u8 {
        match self {
            Self::Scale1 => 0b00,
            Self::Scale2 => 0b01,
            Self::Scale4 => 0b10,
            Self::Scale8 => 0b11,
        }
    }
}

/// Represents the offset for a memory operand.
///
/// # Values
///
/// - `None` - No offset.
/// - `Bit8` - 8-bit signed immediate offset.
/// - `Bit32` - 32-bit signed immediate offset.
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
    /// Converts the offset to a 32-bit signed immediate value.
    ///
    /// # Returns
    ///
    /// * `0` if offset is `None`.
    /// * `Immediate32::from_imm8_sign_extended(offset)` if offset is `Bit8`.
    /// * `offset` if offset is `Bit32`.
    #[inline(always)]
    pub const fn as_sign_extended_imm32(self) -> Immediate32 {
        match self {
            Self::None => Immediate32::from_u32(0),
            Self::Bit8(imm8) => Immediate32::from_imm8_sign_extended(imm8),
            Self::Bit32(imm32) => imm32,
        }
    }
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
    /// Represents a based memory operand, i.e. `[base + offset]`.
    Based { base: GPR, offset: Offset },

    /// Represents a scaled memory operand, i.e. `[index * scale + offset]`.
    ///
    /// # Safety
    ///
    /// `index == GPR::RSP` is not allowed.
    Scaled { index: GPR, scale: Scale, offset: Offset },

    /// Represents a based scaled memory operand, i.e. `[base + index * scale + offset]`.
    ///
    /// # Safety
    ///
    /// `index == GPR::RSP` is not allowed.
    BasedScaled {
        base: GPR,
        index: GPR,
        scale: Scale,
        offset: Offset,
    },

    /// Represents a RIP-relative memory operand, i.e. `[RIP + offset]`.
    RelativeToRIP { offset: Offset },
}

#[must_use]
pub(crate) struct BaseIndexIsExtended {
    pub base_is_extended: bool,
    pub index_is_extended: bool,
}

impl Memory {
    #[inline(always)]
    pub(crate) const fn base_index_is_extended(&self) -> BaseIndexIsExtended {
        match self {
            Self::Based { base, .. } => BaseIndexIsExtended {
                base_is_extended: base.is_extended(),
                index_is_extended: false,
            },
            Self::Scaled { index, .. } => BaseIndexIsExtended {
                base_is_extended: false,
                index_is_extended: index.is_extended(),
            },
            Self::BasedScaled { base, index, .. } => BaseIndexIsExtended {
                base_is_extended: base.is_extended(),
                index_is_extended: index.is_extended(),
            },
            Self::RelativeToRIP { .. } => BaseIndexIsExtended {
                base_is_extended: false,
                index_is_extended: false,
            },
        }
    }
}
