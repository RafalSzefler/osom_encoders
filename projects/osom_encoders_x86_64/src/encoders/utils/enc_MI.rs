use osom_encoders_common::osom_debug_assert;

use crate::{
    encoders::utils::{enc_M::encode_M_gpr_or_memory, helpers::OPERAND_SIZE_OVERRIDE_PREFIX},
    models::{EncodedX86_64Instruction, GPROrMemory, Immediate8, Immediate16, Immediate32},
};

#[cfg(debug_assertions)]
use crate::models::Size;

#[cfg(debug_assertions)]
const fn gpr_size_is_valid(gpr_or_memory: &GPROrMemory, size: Size) -> bool {
    match gpr_or_memory {
        GPROrMemory::GPR { gpr } => gpr.size().equals(size),
        GPROrMemory::Memory { .. } => true,
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 8-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm8_imm8<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit8));
    unsafe {
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm8.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 16-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm16_imm16<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm16: Immediate16,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit16));
    unsafe {
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm16.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 16-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm16_imm8<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit16));
    unsafe {
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm8.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 32-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm32_imm32<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm32: Immediate32,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit32));
    unsafe {
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm32.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 32-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm32_imm8<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit32));
    unsafe {
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm8.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 64-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm64_imm32<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm32: Immediate32,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit64));
    unsafe {
        // This is the same as encode_MI_rm32_imm32.
        // The `encode_M_gpr_or_memory` function already handles
        // 64-bit register encoding.
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm32.encode());
        instr
    }
}

/// # Safety
///
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 64-bit register
/// * * `memory` has to be a valid memory operand
///
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm64_imm8<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    osom_debug_assert!(gpr_size_is_valid(gpr_or_memory, Size::Bit64));
    unsafe {
        // This is the same as encode_MI_rm32_imm32.
        // The `encode_M_gpr_or_memory` function already handles
        // 64-bit register encoding.
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_M_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm8.encode());
        instr
    }
}
