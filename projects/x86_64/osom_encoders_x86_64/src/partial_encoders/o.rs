use crate::models::{EncodedX86_64Instruction, GPR, Size};

use super::core::{OPERAND_SIZE_OVERRIDE_PREFIX, REX_B};

/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
pub const unsafe fn encode(opcode: [u8; 1], gpr: GPR) -> EncodedX86_64Instruction {
    debug_assert!(!gpr.size().equals(Size::Bit8));
    debug_assert!(!gpr.size().equals(Size::Bit32));
    let mut encoded_instruction = EncodedX86_64Instruction::new();
    if gpr.size().equals(Size::Bit16) {
        encoded_instruction.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
    }
    if gpr.is_extended() {
        encoded_instruction.push_array([REX_B.get()]);
    }
    encoded_instruction.push_array([opcode[0] + gpr.lower_3_bits_index()]);
    encoded_instruction
}
