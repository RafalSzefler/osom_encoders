use core::num::NonZero;

use crate::{
    encoders::utils::helpers::{OPERAND_SIZE_OVERRIDE_PREFIX, REX, REX_B, REX_R, REX_W, encode_memory, mod_rm, rex},
    models::{EncodedX86_64Instruction, GPR, GPRKind, GPROrMemory, Memory, Size},
};

/// # Safety
///
/// This function is unsafe because it doesn't validate the operands.
pub const unsafe fn encode_MR<const T: usize>(
    opcode: [u8; T],
    gpr_or_memory: &GPROrMemory,
    gpr: GPR,
) -> EncodedX86_64Instruction {
    unsafe {
        let mut result_instr = EncodedX86_64Instruction::new();
        if gpr.size().equals(Size::Bit16) {
            result_instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        }

        match gpr_or_memory {
            GPROrMemory::GPR { gpr: mem_gpr } => {
                let rex = {
                    const fn unwrap_rex(val: Option<NonZero<u8>>) -> u8 {
                        (if let Some(val) = val { val } else { REX }).get()
                    }
                    let mut rex_result = None;
                    if mem_gpr.index_matches_bit8_high() && mem_gpr.kind().equals(GPRKind::Bit8) {
                        rex_result = Some(REX);
                    }

                    if mem_gpr.is_extended() {
                        let val = unwrap_rex(rex_result);
                        rex_result = Some(NonZero::new_unchecked(val | REX_B.get()));
                    }

                    if mem_gpr.size().equals(Size::Bit64) {
                        let val = unwrap_rex(rex_result);
                        rex_result = Some(NonZero::new_unchecked(val | REX_W.get()));
                    }

                    if gpr.is_extended() {
                        let val = unwrap_rex(rex_result);
                        rex_result = Some(NonZero::new_unchecked(val | REX_R.get()));
                    }

                    rex_result
                };

                if let Some(rex) = rex {
                    result_instr.push_array([rex.get()]);
                }

                result_instr.push_array(opcode);
                result_instr.push_array([mod_rm(0b11, gpr.lower_3_bits_index(), mem_gpr.lower_3_bits_index())]);
            }
            GPROrMemory::Memory { memory } => {
                let ext = memory.base_index_is_extended();
                let base_is_extended = ext.base_is_extended;
                let index_is_extended = ext.index_is_extended;
                let gpr_is_extended = gpr.is_extended();
                let gpr_is_bit64 = gpr.size().equals(Size::Bit64);

                if base_is_extended || index_is_extended || gpr_is_extended || gpr_is_bit64 {
                    let rex = rex(
                        if gpr_is_bit64 { 1 } else { 0 },
                        if gpr_is_extended { 1 } else { 0 },
                        if index_is_extended { 1 } else { 0 },
                        if base_is_extended { 1 } else { 0 },
                    );

                    result_instr.push_array([rex.get()]);
                }

                result_instr.push_array(opcode);

                let result = encode_memory(gpr.lower_3_bits_index(), memory);
                result_instr.push_slice(result.as_slice());
            }
        }

        result_instr
    }
}

/// # Safety
///
/// This function is unsafe because it doesn't validate the operands.
pub const unsafe fn encode_MR_m<const T: usize>(
    opcode: [u8; T],
    memory: &Memory,
    gpr: GPR,
) -> EncodedX86_64Instruction {
    unsafe {
        let mut result_instr = EncodedX86_64Instruction::new();
        if gpr.size().equals(Size::Bit16) {
            result_instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
        }

        let ext = memory.base_index_is_extended();
        let base_is_extended = ext.base_is_extended;
        let index_is_extended = ext.index_is_extended;
        let gpr_is_extended = gpr.is_extended();
        let gpr_is_bit64 = gpr.size().equals(Size::Bit64);

        if base_is_extended || index_is_extended || gpr_is_extended || gpr_is_bit64 {
            let rex = rex(
                if gpr_is_bit64 { 1 } else { 0 },
                if gpr_is_extended { 1 } else { 0 },
                if index_is_extended { 1 } else { 0 },
                if base_is_extended { 1 } else { 0 },
            );

            result_instr.push_array([rex.get()]);
        }

        result_instr.push_array(opcode);

        let result = encode_memory(gpr.lower_3_bits_index(), memory);
        result_instr.push_slice(result.as_slice());

        result_instr
    }
}
