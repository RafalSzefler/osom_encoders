use crate::models::EncodedX86_64Instruction;

/// Encodes a zero operand instruction.
#[inline(always)]
pub const unsafe fn encode<const N: usize>(opcode: [u8; N]) -> EncodedX86_64Instruction {
    EncodedX86_64Instruction::from_array(opcode)
}
