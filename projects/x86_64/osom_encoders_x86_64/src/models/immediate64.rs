#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

use super::{Immediate8, Immediate16, Immediate32};

/// Represents an 64-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate64 {
    value: u64,
}

impl Immediate64 {
    #[inline(always)]
    pub const fn from_imm8_zero_extended(value: Immediate8) -> Self {
        Self::from_u64(value.as_u8() as u64)
    }

    #[inline(always)]
    pub const fn from_imm8_sign_extended(value: Immediate8) -> Self {
        Self::from_i64(value.as_i8() as i64)
    }

    #[inline(always)]
    pub const fn from_imm16_zero_extended(value: Immediate16) -> Self {
        Self::from_u64(value.as_u16() as u64)
    }

    #[inline(always)]
    pub const fn from_imm16_sign_extended(value: Immediate16) -> Self {
        Self::from_i64(value.as_i16() as i64)
    }

    #[inline(always)]
    pub const fn from_imm32_zero_extended(value: Immediate32) -> Self {
        Self::from_u64(value.as_u32() as u64)
    }

    #[inline(always)]
    pub const fn from_imm32_sign_extended(value: Immediate32) -> Self {
        Self::from_i64(value.as_i32() as i64)
    }

    #[inline(always)]
    pub const fn from_u64(value: u64) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i64(value: i64) -> Self {
        Self { value: value as u64 }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_i64(self) -> i64 {
        self.value as i64
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 8] {
        self.value.to_le_bytes()
    }

    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u64() == other.as_u64()
    }
}

impl From<u64> for Immediate64 {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl From<Immediate64> for u64 {
    fn from(value: Immediate64) -> Self {
        value.as_u64()
    }
}

impl From<i64> for Immediate64 {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<Immediate64> for i64 {
    fn from(value: Immediate64) -> Self {
        value.as_i64()
    }
}
