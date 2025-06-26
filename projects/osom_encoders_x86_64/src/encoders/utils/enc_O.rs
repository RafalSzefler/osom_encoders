use osom_encoders_common::osom_debug_assert;

use crate::{
    encoders::utils::helpers::{OPERAND_SIZE_OVERRIDE_PREFIX, REX_B},
    models::{EncodedX86_64Instruction, GPR, Size},
};

/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
pub const unsafe fn encode_O(opcode: u8, gpr: GPR) -> EncodedX86_64Instruction {
    osom_debug_assert!(!gpr.size().equals(Size::Bit8));
    osom_debug_assert!(!gpr.size().equals(Size::Bit32));
    unsafe {
        let mut encoded_instruction = EncodedX86_64Instruction::new();
        if gpr.size().equals(Size::Bit16) {
            encoded_instruction.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        }
        if gpr.is_extended() {
            encoded_instruction.push_array([REX_B.get()]);
        }
        encoded_instruction.push_array([opcode + gpr.lower_3_bits_index()]);
        encoded_instruction
    }
}
