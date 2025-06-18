use osom_encoders_common::FixedBuffer;

/// Represents a binary encoded X64 instruction.
///
/// It is guaranteed to be of size 16. First 15 bytes
/// takes the instruction itself, while last byte is its length.
/// Note that X64 instruction's length is at most 15.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct EncodedX86_64Instruction {
    value: FixedBuffer<15>,
}

impl EncodedX86_64Instruction {
    pub const MAX_SIZE: usize = 15;

    /// Creates a new `EncodedInstruction` with all values set to 0.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it crates an invalid instruction.
    #[inline(always)]
    pub(crate) const unsafe fn new() -> Self {
        Self {
            value: FixedBuffer::new(),
        }
    }

    /// Creates a new `EncodedInstruction` from an array of `N` bytes.
    /// Equivalent to [`EncodedInstruction::new`] followed by
    /// [`EncodedInstruction::push_array`].
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check whether
    /// the underlying buffer has enough space to fit the array.
    /// It is up to the caller to ensure that.
    #[inline(always)]
    pub(crate) const unsafe fn from_array<const N: usize>(array: [u8; N]) -> Self {
        Self {
            value: unsafe { FixedBuffer::from_array(array) },
        }
    }

    /// Pushes an array of `N` bytes to the instruction.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check whether
    /// the underlying buffer has enough space to fit the array.
    /// It is up to the caller to ensure that.
    #[inline(always)]
    pub(crate) const unsafe fn push_array<const N: usize>(&mut self, array: [u8; N]) {
        unsafe { self.value.push_array(array) };
    }

    /// Pushes slice to the instruction.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check whether
    /// the underlying buffer has enough space to fit the slice.
    /// It is up to the caller to ensure that.
    #[inline(always)]
    pub(crate) const unsafe fn push_slice(&mut self, slice: &[u8]) {
        unsafe { self.value.push_slice(slice) };
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        self.value.as_slice()
    }
}
