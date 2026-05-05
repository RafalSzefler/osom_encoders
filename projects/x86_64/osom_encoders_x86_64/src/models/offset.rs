use super::{Immediate8, Immediate32};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u8)]
#[must_use]
pub enum Offset {
    /// No offset is specified.
    None = 0,

    /// 8-bit signed immediate offset.
    Bit8(Immediate8) = 1,

    /// 32-bit signed immediate offset.
    Bit32(Immediate32) = 2,
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
    pub(crate) const fn to_sign_extended_imm32(self) -> Immediate32 {
        match self {
            Self::None => Immediate32::from_u32(0),
            Self::Bit8(imm8) => Immediate32::from_imm8_sign_extended(imm8),
            Self::Bit32(imm32) => imm32,
        }
    }

    /// Creates a new [`Offset`] out of raw `i8`.
    #[inline]
    pub const fn from_i8(value: i8) -> Self {
        Self::Bit8(Immediate8::from_i8(value))
    }

    /// Creates a new [`Offset`] out of raw `u8`.
    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        Self::Bit8(Immediate8::from_u8(value))
    }

    /// Creates a new [`Offset`] out of raw `i32`.
    #[inline]
    pub const fn from_i32(value: i32) -> Self {
        Self::Bit32(Immediate32::from_i32(value))
    }

    /// Creates a new [`Offset`] out of raw `u32`.
    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self::Bit32(Immediate32::from_u32(value))
    }
}

impl From<Immediate32> for Offset {
    fn from(value: Immediate32) -> Self {
        Self::Bit32(value)
    }
}

impl From<Immediate8> for Offset {
    fn from(value: Immediate8) -> Self {
        Self::Bit8(value)
    }
}

impl From<i32> for Offset {
    fn from(value: i32) -> Self {
        Self::Bit32(value.into())
    }
}

impl From<i8> for Offset {
    fn from(value: i8) -> Self {
        Self::Bit8(value.into())
    }
}
