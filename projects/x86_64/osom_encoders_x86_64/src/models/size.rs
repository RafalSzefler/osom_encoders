use core::mem::transmute;

/// Represents all the sizes used by the project from the X64 instruction set.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[repr(u8)]
#[must_use]
pub enum Size {
    Bit8 = 1, // We start from 1 to allow Option<Size> optimization
    Bit16 = 2,
    Bit32 = 3,
    Bit64 = 4,
}

impl Size {
    /// Compares two [`Size`] values for equality.
    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }

    #[inline(always)]
    #[must_use]
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
        debug_assert!(value > 0 && value <= 4, "Invalid size value");
        unsafe { transmute(value) }
    }
}
