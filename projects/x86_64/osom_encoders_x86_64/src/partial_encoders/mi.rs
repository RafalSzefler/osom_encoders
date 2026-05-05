use crate::models::{EncodedX86_64Instruction, GPROrMemory, Immediate8, Immediate16, Immediate32, Size};

use super::m::encode_gpr_or_memory;

use super::core::OPERAND_SIZE_OVERRIDE_PREFIX;

cfg_select! {
    debug_assertions => {
        #[inline]
        const fn gpr_size_is_valid(gpr_or_memory: GPROrMemory, size: Size) {
            match gpr_or_memory {
                GPROrMemory::GPR { gpr } => {
                    if gpr.size().equals(size) {
                        return;
                    }
                }
                GPROrMemory::Memory { .. } => return
            }
            panic!("GPR size mismatch");
        }
    }
    _ => {
        #[inline(always)]
        const fn gpr_size_is_valid(_: GPROrMemory, _: Size) { }
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
pub const unsafe fn encode_rm8_imm8<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit8);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
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
pub const unsafe fn encode_rm16_imm16<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm16: Immediate16,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit16);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
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
pub const unsafe fn encode_rm16_imm8<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit16);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
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
pub const unsafe fn encode_rm32_imm32<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm32: Immediate32,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit32);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
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
pub const unsafe fn encode_rm32_imm8<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit32);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
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
pub const unsafe fn encode_rm64_imm32<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm32: Immediate32,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit64);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
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
pub const unsafe fn encode_rm64_imm8<const N: usize>(
    opcode: [u8; N],
    extended_opcode: u8,
    gpr_or_memory: GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    unsafe {
        gpr_size_is_valid(gpr_or_memory, Size::Bit64);
        let mut instr = EncodedX86_64Instruction::new();
        instr.push_slice(encode_gpr_or_memory(opcode, extended_opcode, gpr_or_memory, true, false).as_slice());
        instr.push_array(imm8.encode());
        instr
    }
}
