//! This module contains the models for the `X86_64` entities, e.g.
//! [`GPR`], [`Memory`] and more.

mod immediate8;
pub use immediate8::*;

mod immediate16;
pub use immediate16::*;

mod immediate32;
pub use immediate32::*;

mod immediate64;
pub use immediate64::*;

mod offset;
pub use offset::*;

mod scale;
pub use scale::*;

mod size;
pub use size::*;

mod gpr_kind;
pub use gpr_kind::*;

mod gpr;
pub use gpr::*;

mod memory;
pub use memory::*;

mod gpr_or_memory;
pub use gpr_or_memory::*;

mod encoded_instruction;
pub use encoded_instruction::*;

mod const_checks;

/// Asserts that an encoded instruction matches the expected bytes.
///
/// # Panics
///
/// Panics if the encoded bytes of `actual` do not equal `expected`.
pub fn assert_encoded_instruction_eq(expected: &[u8], actual: &EncodedX86_64Instruction) {
    assert_eq!(expected, actual.as_slice());
}
