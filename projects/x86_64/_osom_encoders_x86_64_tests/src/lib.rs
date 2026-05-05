//! The purpose of this crate is to provide tests for the `osom_encoders_x86_64`
//! crate by running the generated code at runtime. These tests require the `X86_64`
//! platform and do nothing on other platforms.
#![deny(warnings)]
#![allow(unused_features)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]

use std::marker::PhantomData;

use region::Allocation;

/// Represents a memory region that is executable. This struct
/// is similar to `Vec<u8>` and can freely grow. In addition,
/// by extracting the internal pointer and doing and unsafe
/// transmute, it can be used as a function pointer.
#[must_use]
pub struct ExecutableMemory {
    alloc: Allocation,
    len: usize,
}

#[must_use]
pub struct ExecutablePtr<'a> {
    ptr: *const u8,
    len: usize,
    _phantom: PhantomData<&'a ()>,
}

impl ExecutablePtr<'_> {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn as_ptr(&self) -> *const u8 {
        unsafe {
            region::protect(self.ptr, self.len, region::Protection::EXECUTE)
                .expect("Memory protection should be modifiable");
        };
        self.ptr
    }
}

impl AsRef<[u8]> for ExecutablePtr<'_> {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for ExecutablePtr<'_> {
    fn drop(&mut self) {
        unsafe {
            region::protect(self.ptr, self.len, region::Protection::READ_WRITE)
                .expect("Memory protection should be modifiable");
        };
    }
}

impl ExecutableMemory {
    /// Creates a new [`ExecutableMemory`].
    ///
    /// # Panics
    ///
    /// If the region allocation fails, a panic is raised.
    pub fn new() -> Self {
        let pz = region::page::size();
        let pz = std::cmp::max(4096, pz);
        let region = region::alloc(pz, region::Protection::READ_WRITE).expect("Allocated region");
        Self { alloc: region, len: 0 }
    }

    /// Pushes data to the [`ExecutableMemory`].
    ///
    /// # Panics
    ///
    /// If the data is too large and the underlying buffer overflows, a panic is raised.
    pub fn push<T: AsRef<[u8]>>(&mut self, data: T) {
        let data = data.as_ref();
        let new_len = self.len + data.len();
        self.grow_if_necessary(new_len);

        unsafe {
            let ptr = self.alloc.as_mut_ptr::<u8>().add(self.len);
            ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
        }

        self.len = new_len;
    }

    pub fn as_exe(&self) -> ExecutablePtr<'_> {
        ExecutablePtr {
            ptr: self.alloc.as_ptr(),
            len: self.len,
            _phantom: PhantomData,
        }
    }

    fn grow_if_necessary(&mut self, new_len: usize) {
        if new_len < self.alloc.len() {
            return;
        }

        let new_capacity = (new_len + 1).next_power_of_two();
        assert!(new_capacity < i32::MAX as usize, "Max buffer size exceeded.");
        let mut new_region = region::alloc(new_capacity, region::Protection::READ_WRITE).expect("Allocated region");

        unsafe {
            new_region
                .as_mut_ptr::<u8>()
                .copy_from_nonoverlapping(self.alloc.as_ptr::<u8>(), self.len);
        }

        std::mem::swap(&mut self.alloc, &mut new_region);
    }
}

impl Default for ExecutableMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for ExecutableMemory {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.alloc.as_ptr(), self.len) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executable_memory() {
        let mut memory = ExecutableMemory::new();
        memory.push(&[0x00, 0x01, 0x02, 0x03]);
        assert_eq!(memory.as_ref(), &[0x00, 0x01, 0x02, 0x03]);

        memory.push(&[0x04, 0x05, 0x06, 0x07]);
        assert_eq!(memory.as_ref(), &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    }

    #[test]
    fn test_realloc() {
        const DATA_SIZE: usize = 100000;
        let mut memory = ExecutableMemory::new();
        let mut data = Vec::with_capacity(DATA_SIZE);
        for i in 0..DATA_SIZE {
            data.push(i as u8);
        }
        memory.push(&data);
        assert_eq!(memory.as_ref(), &data);
    }
}
