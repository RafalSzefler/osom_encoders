//! # Content
//!
//! Holds encoders for instructions valid in both 32 and 64-bit modes.
//! Manually handled file.

pub use super::universal_gen::*;

use osom_encoders_common::osom_assert;

use crate::models::EncodedX86Instruction;

/// No operation for given length.
///
/// Uses Intel's recommended multi-byte NOP sequences. Note that
/// [`encode_nop_with_length(1)`] call is equivalent to [`encode_nop()`],
/// although slightly less efficient.
///
/// # Safety
///
/// The caller *must* ensure that `length` is in the `1..=9` range.
/// Otherwise the behaviour is undefined.
#[inline]
pub const unsafe fn encode_nop_with_length(length: u8) -> EncodedX86Instruction {
    osom_assert!(length >= 1 && length <= 9);
    unsafe {
        match length {
            1 => EncodedX86Instruction::from_array([0x90]),
            2 => EncodedX86Instruction::from_array([0x66, 0x90]),
            3 => EncodedX86Instruction::from_array([0x0F, 0x1F, 0x00]),
            4 => EncodedX86Instruction::from_array([0x0F, 0x1F, 0x40, 0x00]),
            5 => EncodedX86Instruction::from_array([0x0F, 0x1F, 0x44, 0x00, 0x00]),
            6 => EncodedX86Instruction::from_array([0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00]),
            7 => EncodedX86Instruction::from_array([0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00]),
            8 => EncodedX86Instruction::from_array([0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
            9 => EncodedX86Instruction::from_array([0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}
