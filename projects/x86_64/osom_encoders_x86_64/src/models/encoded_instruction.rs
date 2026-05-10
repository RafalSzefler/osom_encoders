use _osom_encoders_common::fixed_bytes::FixedBytes;

/// Represents a binary encoded `X86_64` instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct EncodedX86_64Instruction {
    value: FixedBytes<15>,
}

impl EncodedX86_64Instruction {
    /// The maximum size (in bytes) of an encoded `X86_64` instruction.
    pub const MAX_SIZE: usize = 15;

    /// Represents the [`EncodedX86_64Instruction`] as a slice of bytes.
    #[inline(always)]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        self.value.as_slice()
    }

    /// Represents the [`EncodedX86_64Instruction`] as a slice of bytes.
    #[inline(always)]
    #[must_use]
    pub const fn as_slice_mut(&mut self) -> &mut [u8] {
        self.value.as_slice_mut()
    }

    /// Creates a new [`EncodedX86_64Instruction`] from an array of `SIZE` bytes.
    ///
    /// # Panics
    ///
    /// This function panics if the array is too large.
    #[inline]
    pub const fn from_array<const SIZE: usize>(array: [u8; SIZE]) -> Self {
        if SIZE == 0 {
            return Self::new();
        }

        const {
            assert!(SIZE <= Self::MAX_SIZE, "SIZE must be less than or equal to MAX_SIZE");
        }
        let mut result = Self::new();
        result.push_array(array);
        result
    }

    /// Creates a new, empty [`EncodedX86_64Instruction`].
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            value: FixedBytes::new(),
        }
    }

    /// Pushes an array of `N` bytes to the instruction.
    ///
    /// # Panics
    ///
    /// This function panics if the array is too large.
    #[inline]
    pub(crate) const fn push_array<const SIZE: usize>(&mut self, array: [u8; SIZE]) {
        if SIZE == 0 {
            return;
        }

        const {
            assert!(SIZE <= Self::MAX_SIZE, "SIZE must be less than or equal to MAX_SIZE");
        }
        self.value.push_array(array);
    }

    /// Pushes slice to the instruction.
    ///
    /// # Panics
    ///
    /// This function panics if the slice is too large.
    #[inline]
    pub(crate) const fn push_slice(&mut self, slice: &[u8]) {
        self.value.push_slice(slice);
    }
}

impl AsRef<[u8]> for EncodedX86_64Instruction {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for EncodedX86_64Instruction {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}
