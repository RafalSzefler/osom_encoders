/// Returns a slice of `T` from `slice` starting at `start` and of length `length`.
///
/// # Safety
///
/// The caller must ensure that `start + length` is within the bounds of `slice`.
/// Otherwise, the behavior is undefined.
#[inline]
pub const unsafe fn const_slice<T>(slice: &[T], start: usize, length: usize) -> &[T] {
    unsafe {
        let ptr = slice.as_ptr().add(start);
        core::slice::from_raw_parts(ptr, length)
    }
}

/// Returns a mutable slice of `T` from `slice` starting at `start` and of length `length`.
///
/// # Safety
///
/// The caller must ensure that `start + length` is within the bounds of `slice`.
/// Otherwise, the behavior is undefined.
#[inline]
pub const unsafe fn const_slice_mut<T>(slice: &mut [T], start: usize, length: usize) -> &mut [T] {
    unsafe {
        let ptr = slice.as_mut_ptr().add(start);
        core::slice::from_raw_parts_mut(ptr, length)
    }
}
