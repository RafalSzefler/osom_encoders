use crate::models::EncodedX86_64Instruction;

/// Encodes a zero operand instruction. This function can be used
/// to encode full instruction with raw bytes directly.
#[inline(always)]
pub const unsafe fn encode<const N: usize>(opcode: [u8; N]) -> EncodedX86_64Instruction {
    EncodedX86_64Instruction::from_array(opcode)
}
