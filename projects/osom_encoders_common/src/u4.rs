use crate::macros::osom_assert;

/// Representation of a 4-bit unsigned integer. Internally stored as a `u8`,
/// but with value validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[must_use]
pub struct U4 {
    value: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NewU4Error;

impl U4 {
    /// Creates a new `u4` from a `u8` value.
    ///
    /// # Safety
    ///
    /// The value must be in the range `0..=15`, otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        osom_assert!(value <= 15);
        unsafe { core::hint::assert_unchecked(value <= 15) };
        Self { value }
    }

    /// Creates a new `u4` from a `u8` value.
    ///
    /// # Safety
    ///
    /// The value must be in the range `0..=15`, otherwise the behavior is undefined.
    #[inline]
    pub const fn new(value: u8) -> Result<Self, NewU4Error> {
        if value > 15 {
            return Err(NewU4Error);
        }

        Ok(unsafe { Self::new_unchecked(value) })
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        unsafe { core::hint::assert_unchecked(self.value <= 15) };
        self.value
    }

    #[inline(always)]
    pub const fn binary_and(self, other: Self) -> Self {
        Self {
            value: self.value & other.value,
        }
    }

    #[inline(always)]
    pub const fn binary_or(self, other: Self) -> Self {
        Self {
            value: self.value | other.value,
        }
    }

    #[inline(always)]
    pub const fn binary_xor(self, other: Self) -> Self {
        Self {
            value: self.value ^ other.value,
        }
    }

    #[inline(always)]
    pub const fn binary_not(self) -> Self {
        Self {
            value: (!self.value) & 0b1111,
        }
    }

    #[inline(always)]
    pub const fn binary_shl(self, other: u8) -> Self {
        Self {
            value: (self.value << other) & 0b1111,
        }
    }

    #[inline(always)]
    pub const fn binary_shr(self, other: u8) -> Self {
        Self {
            value: self.value >> other,
        }
    }
}

impl core::ops::BitAnd for U4 {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        self.binary_and(other)
    }
}

impl core::ops::BitOr for U4 {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        self.binary_or(other)
    }
}

impl core::ops::BitXor for U4 {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        self.binary_xor(other)
    }
}

impl core::ops::Not for U4 {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.binary_not()
    }
}

impl core::ops::Shl<u8> for U4 {
    type Output = Self;

    fn shl(self, other: u8) -> Self::Output {
        self.binary_shl(other)
    }
}

impl core::ops::Shr<u8> for U4 {
    type Output = Self;

    fn shr(self, other: u8) -> Self::Output {
        self.binary_shr(other)
    }
}

pub mod u4_values {
    use super::U4;

    pub const ZERO: U4 = unsafe { U4::new_unchecked(0) };
    pub const ONE: U4 = unsafe { U4::new_unchecked(1) };
    pub const TWO: U4 = unsafe { U4::new_unchecked(2) };
    pub const THREE: U4 = unsafe { U4::new_unchecked(3) };
    pub const FOUR: U4 = unsafe { U4::new_unchecked(4) };
    pub const FIVE: U4 = unsafe { U4::new_unchecked(5) };
    pub const SIX: U4 = unsafe { U4::new_unchecked(6) };
    pub const SEVEN: U4 = unsafe { U4::new_unchecked(7) };
    pub const EIGHT: U4 = unsafe { U4::new_unchecked(8) };
    pub const NINE: U4 = unsafe { U4::new_unchecked(9) };
    pub const TEN: U4 = unsafe { U4::new_unchecked(10) };
    pub const ELEVEN: U4 = unsafe { U4::new_unchecked(11) };
    pub const TWELVE: U4 = unsafe { U4::new_unchecked(12) };
    pub const THIRTEEN: U4 = unsafe { U4::new_unchecked(13) };
    pub const FOURTEEN: U4 = unsafe { U4::new_unchecked(14) };
    pub const FIFTEEN: U4 = unsafe { U4::new_unchecked(15) };
}

const _INVARIANTS: () = {
    assert!(size_of::<U4>() == 1, "Expected U4 to be 1 byte");
};
