use crate::models::{EncodedX86_64Instruction, Immediate8, Immediate16, Immediate32};

use super::core::{OPERAND_SIZE_OVERRIDE_PREFIX, REX_W};

/// Encodes I encoding with an 8-bit immediate value.
#[inline]
pub const unsafe fn encode_imm8<const N: usize>(opcode: [u8; N], imm8: Immediate8) -> EncodedX86_64Instruction {
    let mut instr = EncodedX86_64Instruction::from_array(opcode);
    instr.push_array(imm8.encode());
    instr
}

/// Encodes I encoding with an 16-bit immediate value and operand size override.
#[inline]
pub const unsafe fn encode_imm16_oso<const N: usize>(opcode: [u8; N], imm16: Immediate16) -> EncodedX86_64Instruction {
    let mut instr = EncodedX86_64Instruction::from_array([OPERAND_SIZE_OVERRIDE_PREFIX.get()]);
    instr.push_array(opcode);
    instr.push_array(imm16.encode());
    instr
}

/// Encodes I encoding with an 16-bit immediate value.
#[inline]
pub const unsafe fn encode_imm16<const N: usize>(opcode: [u8; N], imm16: Immediate16) -> EncodedX86_64Instruction {
    let mut instr = EncodedX86_64Instruction::from_array(opcode);
    instr.push_array(imm16.encode());
    instr
}

/// Encodes I encoding with an 32-bit immediate value.
#[inline]
pub const unsafe fn encode_imm32<const N: usize>(opcode: [u8; N], imm32: Immediate32) -> EncodedX86_64Instruction {
    let mut instr = EncodedX86_64Instruction::from_array(opcode);
    instr.push_array(imm32.encode());
    instr
}

/// Encodes I encoding with an 32-bit immediate value and REX.W prefix.
#[inline]
pub const unsafe fn encode_imm32_rexw<const N: usize>(opcode: [u8; N], imm32: Immediate32) -> EncodedX86_64Instruction {
    let mut instr = EncodedX86_64Instruction::from_array([REX_W.get()]);
    instr.push_array(opcode);
    instr.push_array(imm32.encode());
    instr
}
