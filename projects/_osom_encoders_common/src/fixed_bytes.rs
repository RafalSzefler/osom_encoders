use core::hash::{Hash, Hasher};

use crate::const_helpers::const_slice_mut;

/// Represents a fixed-size array of bytes. This array is optimized
/// for size. The size of the array cannot exceed `u8::MAX`.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct FixedBytes<const CAPACITY: usize> {
    data: [u8; CAPACITY],
    length: u8,
}

/// Represents an error that can occur when pushing a value to a [`FixedBytes`].
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub enum FixedBytesError {
    /// The capacity of the [`FixedBytes`] has been exceeded.
    CapacityExceeded = 0,
}

impl<const CAPACITY: usize> FixedBytes<CAPACITY> {
    /// Creates a new, empty [`FixedBytes`].
    #[inline]
    pub const fn new() -> Self {
        const {
            assert!(CAPACITY < u8::MAX as usize, "CAPACITY must be less than u8::MAX");
        }

        #[allow(clippy::uninit_assumed_init)]
        Self {
            length: 0,
            data: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
        }
    }

    /// Represents the [`FixedBytes`] as a slice of bytes.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), self.length as usize) }
    }

    /// Represents the [`FixedBytes`] as a slice of bytes.
    #[inline]
    #[must_use]
    pub const fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data.as_mut_ptr(), self.length as usize) }
    }

    /// Pushes an array of `SIZE` bytes to the [`FixedBytes`].
    ///
    /// # Panics
    ///
    /// This function panics if the array is too large.
    #[inline]
    pub const fn push_array<const SIZE: usize>(&mut self, value: [u8; SIZE]) {
        assert!(self.try_push_array(value).is_ok(), "Passed array is too large");
    }

    /// Pushes a slice of bytes to the [`FixedBytes`].
    ///
    /// # Panics
    ///
    /// This function panics if the slice is too large.
    #[inline]
    pub const fn push_slice(&mut self, value: &[u8]) {
        assert!(self.try_push_slice(value).is_ok(), "Passed slice is too large");
    }

    /// Tries to push an array of `SIZE` bytes to the [`FixedBytes`].
    ///
    /// # Errors
    ///
    /// Returns [`FixedBytesError::CapacityExceeded`] if the capacity of the [`FixedBytes`] has been exceeded.
    pub const fn try_push_array<const SIZE: usize>(&mut self, value: [u8; SIZE]) -> Result<(), FixedBytesError> {
        const {
            assert!(SIZE <= CAPACITY);
        }
        let length = self.length as usize;
        if unsafe { length.unchecked_add(SIZE) } > CAPACITY {
            return Err(FixedBytesError::CapacityExceeded);
        }
        let slice = unsafe { const_slice_mut(&mut self.data, length, SIZE) };
        slice.copy_from_slice(&value);

        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            self.length = self.length.unchecked_add(SIZE as u8);
        }

        Ok(())
    }

    /// Tries to push a slice of bytes to the [`FixedBytes`].
    ///
    /// # Errors
    ///
    /// Returns [`FixedBytesError::CapacityExceeded`] if the capacity of the [`FixedBytes`] has been exceeded.
    pub const fn try_push_slice(&mut self, value: &[u8]) -> Result<(), FixedBytesError> {
        if value.len() > CAPACITY {
            return Err(FixedBytesError::CapacityExceeded);
        }

        let length = self.length as usize;
        if unsafe { length.unchecked_add(value.len()) } > CAPACITY {
            return Err(FixedBytesError::CapacityExceeded);
        }
        let slice = unsafe { const_slice_mut(&mut self.data, length, value.len()) };
        slice.copy_from_slice(value);

        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            self.length = self.length.unchecked_add(value.len() as u8);
        }

        Ok(())
    }
}

impl<const CAPACITY: usize> Default for FixedBytes<CAPACITY> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> AsRef<[u8]> for FixedBytes<CAPACITY> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const CAPACITY: usize> Hash for FixedBytes<CAPACITY> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}
