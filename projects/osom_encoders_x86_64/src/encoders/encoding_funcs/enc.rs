//! # Content
//!
//! Holds encoders for instructions valid in 64-bit X86 instruction set.
//! Manually handled file.
#![allow(clippy::wildcard_imports)]
use crate::models::EncodedX86_64Instruction;

/// Holds encoders not directly defined in the Intel's manual.
pub mod miscellaneous {
    use super::*;

    /// No operation for given length. It is guaranteed that the returned
    /// value is of the passed `length`.
    ///
    /// Uses Intel's recommended multi-byte NOP sequences. Note that
    /// [`encode_nop_with_length(1)`][encode_nop_with_length] call
    /// is equivalent to [`encode_nop()`][crate::encoders::singleton::encode_nop],
    /// although slightly less efficient.
    ///
    /// # Panics
    ///
    /// The caller *must* ensure that `length` is in the `1..=9` range.
    /// Otherwise the function panics.
    #[inline]
    pub const fn encode_nop_with_length(length: u8) -> EncodedX86_64Instruction {
        unsafe {
            match length {
                1 => EncodedX86_64Instruction::from_array([0x90]),
                2 => EncodedX86_64Instruction::from_array([0x66, 0x90]),
                3 => EncodedX86_64Instruction::from_array([0x0F, 0x1F, 0x00]),
                4 => EncodedX86_64Instruction::from_array([0x0F, 0x1F, 0x40, 0x00]),
                5 => EncodedX86_64Instruction::from_array([0x0F, 0x1F, 0x44, 0x00, 0x00]),
                6 => EncodedX86_64Instruction::from_array([0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00]),
                7 => EncodedX86_64Instruction::from_array([0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00]),
                8 => EncodedX86_64Instruction::from_array([0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
                9 => EncodedX86_64Instruction::from_array([0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
                _ => panic!("Invalid length for encode_nop_with_length call. Expected u8 in 1..=9 range."),
            }
        }
    }
}
