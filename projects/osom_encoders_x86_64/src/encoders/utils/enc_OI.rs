use osom_encoders_common::osom_debug_assert;

use crate::{
    encoders::utils::helpers::{OPERAND_SIZE_OVERRIDE_PREFIX, REX, REX_B, rex},
    models::{EncodedX86_64Instruction, GPR, GPRKind, Immediate8, Immediate16, Immediate32, Immediate64},
};

#[allow(unused_imports)]
use crate::models::Size;

/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
pub const unsafe fn encode_OI_r8_imm8(opcode: u8, gpr: GPR, imm8: Immediate8) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr.size().equals(Size::Bit8));
    unsafe {
        let mut encoded_instruction = EncodedX86_64Instruction::new();
        if gpr.is_extended() {
            encoded_instruction.push_array([REX_B.get()]);
        } else if gpr.index_matches_bit8_high() && gpr.kind().equals(GPRKind::Bit8) {
            encoded_instruction.push_array([REX.get()]);
        }

        let opcode_byte = opcode + gpr.lower_3_bits_index();
        encoded_instruction.push_array([opcode_byte]);
        encoded_instruction.push_array(imm8.encode());

        encoded_instruction
    }
}

/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
pub const unsafe fn encode_OI_r16_imm16(opcode: u8, gpr: GPR, imm16: Immediate16) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr.size().equals(Size::Bit16));
    unsafe {
        let mut encoded_instruction = EncodedX86_64Instruction::from_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        if gpr.is_extended() {
            encoded_instruction.push_array([REX_B.get()]);
        }

        let opcode_byte = opcode + gpr.lower_3_bits_index();
        encoded_instruction.push_array([opcode_byte]);
        encoded_instruction.push_array(imm16.encode());

        encoded_instruction
    }
}

/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
pub const unsafe fn encode_OI_r32_imm32(opcode: u8, gpr: GPR, imm32: Immediate32) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr.size().equals(Size::Bit32));
    unsafe {
        let mut encoded_instruction = EncodedX86_64Instruction::new();
        if gpr.is_extended() {
            encoded_instruction.push_array([REX_B.get()]);
        }

        let opcode_byte = opcode + gpr.lower_3_bits_index();
        encoded_instruction.push_array([opcode_byte]);
        encoded_instruction.push_array(imm32.encode());

        encoded_instruction
    }
}

/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
pub const unsafe fn encode_OI_r64_imm64(opcode: u8, gpr: GPR, imm64: Immediate64) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr.size().equals(Size::Bit64));
    unsafe {
        let mut encoded_instruction = EncodedX86_64Instruction::new();

        let rex_prefix = rex(1, 0, 0, if gpr.is_extended() { 1 } else { 0 });

        encoded_instruction.push_array([rex_prefix.get()]);

        let opcode_byte = opcode + gpr.lower_3_bits_index();
        encoded_instruction.push_array([opcode_byte]);
        encoded_instruction.push_array(imm64.encode());

        encoded_instruction
    }
}
