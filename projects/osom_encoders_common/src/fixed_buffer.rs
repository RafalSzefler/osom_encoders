use crate::macros::osom_debug_assert;

/// Similar to vector but with fixed, constant capacity.
///
/// It is guaranteed to be of size `CAPACITY+1`. First `CAPACITY` bytes
/// takes the buffer itself, while last byte is its length.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[must_use]
pub struct FixedBuffer<const CAPACITY: usize> {
    value: [u8; CAPACITY],
    len: u8,
}

impl<const CAPACITY: usize> FixedBuffer<CAPACITY> {
    const _VALIDATE: () = {
        assert!(
            CAPACITY <= u8::MAX as usize,
            "BYTE_SIZE must be less than or equal to 255."
        );
    };

    /// Creates a new `FixedBuffer` with all values set to 0.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            value: [0; CAPACITY],
            len: 0,
        }
    }

    /// Creates a new `FixedBuffer` from an array of `N` bytes.
    /// Equivalent to [`FixedBuffer::new`] followed by
    /// [`FixedBuffer::push_array`].
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check whether
    /// the underlying buffer has enough space to fit the array.
    /// It is up to the caller to ensure that.
    #[inline(always)]
    pub const unsafe fn from_array<const N: usize>(array: [u8; N]) -> Self {
        unsafe {
            let mut buffer = Self::new();
            buffer.push_array(array);
            buffer
        }
    }

    /// Pushes an array of `N` bytes to the buffer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check whether
    /// the underlying buffer has enough space to fit the array.
    /// It is up to the caller to ensure that.
    #[inline(always)]
    pub const unsafe fn push_array<const N: usize>(&mut self, array: [u8; N]) {
        assert!(N <= CAPACITY);
        assert!(self.len as usize + N <= CAPACITY);

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

    /// Pushes slice to the buffer.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it doesn't check whether
    /// the underlying buffer has enough space to fit the slice.
    /// It is up to the caller to ensure that.
    #[inline(always)]
    pub const unsafe fn push_slice(&mut self, slice: &[u8]) {
        osom_debug_assert!(slice.len() + self.len as usize <= CAPACITY);

        let slice_len = slice.len();
        let mut dst_ptr = unsafe { self.value.as_mut_ptr().add(self.len as usize) };
        let mut src_ptr = slice.as_ptr();

        let mut idx = 0;
        while idx < slice_len {
            unsafe {
                dst_ptr.write(src_ptr.read());
                dst_ptr = dst_ptr.add(1);
                src_ptr = src_ptr.add(1);
            }
            idx += 1;
        }

        #[allow(clippy::cast_possible_truncation)]
        let n8 = slice_len as u8;
        self.len += n8;
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        let len = self.len as usize;
        unsafe {
            core::hint::assert_unchecked(len <= CAPACITY);
            core::slice::from_raw_parts(self.value.as_ptr(), len)
        }
    }
}

impl<const CAPACITY: usize> Default for FixedBuffer<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}
