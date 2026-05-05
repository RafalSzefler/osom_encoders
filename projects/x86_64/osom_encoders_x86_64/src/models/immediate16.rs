#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

use super::Immediate8;

/// Represents an 16-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate16 {
    value: u16,
}

impl Immediate16 {
    #[inline(always)]
    pub const fn from_imm8_zero_extended(value: Immediate8) -> Self {
        Self::from_u16(value.as_u8() as u16)
    }

    #[inline(always)]
    pub const fn from_imm8_sign_extended(value: Immediate8) -> Self {
        Self::from_i16(value.as_i8() as i16)
    }

    #[inline(always)]
    pub const fn from_u16(value: u16) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i16(value: i16) -> Self {
        Self { value: value as u16 }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_i16(self) -> i16 {
        self.value as i16
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 2] {
        self.value.to_le_bytes()
    }

    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u16() == other.as_u16()
    }
}

impl From<u16> for Immediate16 {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

impl From<Immediate16> for u16 {
    fn from(value: Immediate16) -> Self {
        value.as_u16()
    }
}

impl From<i16> for Immediate16 {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<Immediate16> for i16 {
    fn from(value: Immediate16) -> Self {
        value.as_i16()
    }
}
