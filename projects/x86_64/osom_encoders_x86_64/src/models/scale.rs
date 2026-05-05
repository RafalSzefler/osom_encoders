use core::mem::transmute;

/// Represents the scale factor for the index register in a memory operand.
///
/// # Values
///
/// - `Scale1` - The index register is multiplied by 1.
/// - `Scale2` - The index register is multiplied by 2.
/// - `Scale4` - The index register is multiplied by 4.
/// - `Scale8` - The index register is multiplied by 8.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u8)]
#[must_use]
pub enum Scale {
    Scale1 = 1, // We start from 1 to allow Option<Scale> optimization
    Scale2 = 2,
    Scale4 = 3,
    Scale8 = 4,
}

impl Scale {
    /// Compares two [`Scale`] values for equality.
    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }

    #[inline(always)]
    pub(crate) const fn as_u8(self) -> u8 {
        unsafe {
            let result = transmute::<Self, u8>(self);
            core::hint::assert_unchecked(result > 0);
            core::hint::assert_unchecked(result <= 4);
            result
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn from_u8(value: u8) -> Self {
        debug_assert!(value > 0 && value <= 4, "Invalid scale value");
        unsafe { transmute(value) }
    }

    #[must_use]
    pub(crate) const fn index(self) -> u8 {
        match self {
            Self::Scale1 => 0,
            Self::Scale2 => 1,
            Self::Scale4 => 2,
            Self::Scale8 => 3,
        }
    }
}
