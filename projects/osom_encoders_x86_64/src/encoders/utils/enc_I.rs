use crate::models::{EncodedX86_64Instruction, Immediate8, Immediate16, Immediate32};

use super::helpers::{OPERAND_SIZE_OVERRIDE_PREFIX, REX_W};

/// # Safety
///
/// This function is unsafe because it doesn't validate the opcode.
/// It is up to the caller to ensure that the opcode is valid.
pub const unsafe fn encode_I_imm8<const T: usize>(opcode: [u8; T], imm8: Immediate8) -> EncodedX86_64Instruction {
    unsafe {
        let mut instr = EncodedX86_64Instruction::from_array(opcode);
        instr.push_array(imm8.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because it doesn't validate the opcode.
/// It is up to the caller to ensure that the opcode is valid.
pub const unsafe fn encode_I_imm16_operand_size_override<const T: usize>(
    opcode: [u8; T],
    imm16: Immediate16,
) -> EncodedX86_64Instruction {
    unsafe {
        let mut instr = EncodedX86_64Instruction::from_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        instr.push_array(opcode);
        instr.push_array(imm16.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because it doesn't validate the opcode.
/// It is up to the caller to ensure that the opcode is valid.
pub const unsafe fn encode_I_imm16<const T: usize>(opcode: [u8; T], imm16: Immediate16) -> EncodedX86_64Instruction {
    unsafe {
        let mut instr = EncodedX86_64Instruction::from_array(opcode);
        instr.push_array(imm16.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because it doesn't validate the opcode.
/// It is up to the caller to ensure that the opcode is valid.
pub const unsafe fn encode_I_imm32<const T: usize>(opcode: [u8; T], imm32: Immediate32) -> EncodedX86_64Instruction {
    unsafe {
        let mut instr = EncodedX86_64Instruction::from_array(opcode);
        instr.push_array(imm32.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because it doesn't validate the opcode.
/// It is up to the caller to ensure that the opcode is valid.
pub const unsafe fn encode_I_imm32_prefix_rex_w<const T: usize>(
    opcode: [u8; T],
    imm32: Immediate32,
) -> EncodedX86_64Instruction {
    unsafe {
        let mut instr = EncodedX86_64Instruction::from_array([REX_W.get()]);
        instr.push_array(opcode);
        instr.push_array(imm32.encode());
        instr
    }
}
