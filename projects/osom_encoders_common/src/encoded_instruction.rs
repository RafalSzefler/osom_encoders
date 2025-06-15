use crate::macros::osom_debug_assert;

/// Represents a binary encoded instruction.
///
/// It is guaranteed to be of size `BYTE_SIZE+1`. First `BYTE_SIZE` bytes
/// takes the instruction itself, while last byte is its length.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[must_use]
pub struct EncodedInstruction<const BYTE_SIZE: usize> {
    value: [u8; BYTE_SIZE],
    len: u8,
}

impl<const BYTE_SIZE: usize> EncodedInstruction<BYTE_SIZE> {
    const _VALIDATE: () = {
        assert!(
            BYTE_SIZE <= u8::MAX as usize,
            "BYTE_SIZE must be less than or equal to 255."
        );
    };

    /// Creates a new `EncodedInstruction` with all values set to 0.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it crates an invalid instruction.
    #[inline(always)]
    pub const unsafe fn new() -> Self {
        Self {
            value: [0; BYTE_SIZE],
            len: 0,
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
    pub const unsafe fn from_array<const N: usize>(array: [u8; N]) -> Self {
        unsafe {
            let mut instruction = Self::new();
            instruction.push_array(array);
            instruction
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
    pub const unsafe fn push_array<const N: usize>(&mut self, array: [u8; N]) {
        osom_debug_assert!(N <= BYTE_SIZE);
        osom_debug_assert!(self.len as usize + N <= BYTE_SIZE);

        let mut ptr = unsafe { self.value.as_mut_ptr().add(self.len as usize) };
        let mut idx = 0;
        while idx < N {
            unsafe {
                ptr.write(array[idx]);
                ptr = ptr.add(1);
            }
            idx += 1;
        }

        #[allow(clippy::cast_possible_truncation)]
        let n8 = N as u8;

        self.len += n8;
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        let len = self.len as usize;
        unsafe {
            core::hint::assert_unchecked(len <= BYTE_SIZE);
            core::slice::from_raw_parts(self.value.as_ptr(), len)
        }
    }
}
