#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

/// Represents an 8-bit immediate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct Immediate8 {
    value: u8,
}

impl Immediate8 {
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub const fn from_i8(value: i8) -> Self {
        Self { value: value as u8 }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.value
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_i8(self) -> i8 {
        self.value as i8
    }

    #[inline(always)]
    #[must_use]
    pub const fn encode(self) -> [u8; 1] {
        [self.value]
    }

    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl From<u8> for Immediate8 {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<Immediate8> for u8 {
    fn from(value: Immediate8) -> Self {
        value.as_u8()
    }
}

impl From<i8> for Immediate8 {
    fn from(value: i8) -> Self {
        Self::from_i8(value)
    }
}

impl From<Immediate8> for i8 {
    fn from(value: Immediate8) -> Self {
        value.as_i8()
    }
}
