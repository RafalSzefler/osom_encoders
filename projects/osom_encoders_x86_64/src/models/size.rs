use osom_encoders_common::osom_debug_assert;

/// Represents all the sizes used by the project from the X64 instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[must_use]
pub enum Size {
    Bit8 = 1, // We start from 1 to allow Option<Size> optimization
    Bit16,
    Bit32,
    Bit64,
}

impl Size {
    #[inline(always)]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        let result = self as u8;
        unsafe { core::hint::assert_unchecked(result <= 15) };
        result
    }

    /// Creates a new `Size` from a `u8` index.
    ///
    /// # Safety
    ///
    /// The index must be in the range `1..=4`, otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(index: u8) -> Self {
        osom_debug_assert!(index > 0 && index <= 4);
        unsafe { core::mem::transmute(index) }
    }

    /// Creates a new `Size` from a `u8` index.
    ///
    /// # Returns
    ///
    /// - `Some(Size)` if the `u8` is in the range `1..=4`
    /// - `None` if the `u8` is 0 or greater than 4
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value == 0 || value > 4 {
            return None;
        }
        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    /// The same as `==` operator but const.
    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl core::hash::Hash for Size {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u8().hash(state);
    }
}
