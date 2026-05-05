#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

use super::{Immediate8, Immediate16};

/// Represents an 32-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate32 {
    value: u32,
}

impl Immediate32 {
    #[inline(always)]
    pub const fn from_imm8_zero_extended(value: Immediate8) -> Self {
        Self::from_u32(value.as_u8() as u32)
    }

    #[inline(always)]
    pub const fn from_imm8_sign_extended(value: Immediate8) -> Self {
        Self::from_i32(value.as_i8() as i32)
    }

    #[inline(always)]
    pub const fn from_imm16_zero_extended(value: Immediate16) -> Self {
        Self::from_u32(value.as_u16() as u32)
    }

    #[inline(always)]
    pub const fn from_imm16_sign_extended(value: Immediate16) -> Self {
        Self::from_i32(value.as_i16() as i32)
    }

    #[inline(always)]
    pub const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i32(value: i32) -> Self {
        Self { value: value as u32 }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self.value as i32
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 4] {
        self.value.to_le_bytes()
    }

    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u32() == other.as_u32()
    }
}

impl From<u32> for Immediate32 {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<Immediate32> for u32 {
    fn from(value: Immediate32) -> Self {
        value.as_u32()
    }
}

impl From<i32> for Immediate32 {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

impl From<Immediate32> for i32 {
    fn from(value: Immediate32) -> Self {
        value.as_i32()
    }
}
