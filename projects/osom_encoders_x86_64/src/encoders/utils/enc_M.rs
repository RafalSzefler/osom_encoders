use core::num::NonZero;

use crate::{
    encoders::utils::helpers::{OPERAND_SIZE_OVERRIDE_PREFIX, REX, REX_B, REX_W, encode_memory, mod_rm, rex},
    models::{EncodedX86_64Instruction, GPRKind, GPROrMemory, Size},
};

pub const unsafe fn encode_M_gpr_or_memory<const T: usize>(
    opcode: [u8; T],
    extended_opcode: u8,
    gpr_or_memory: &GPROrMemory,
    bit64_requires_rex_w: bool,
    bit16_requires_os_prefix: bool,
) -> EncodedX86_64Instruction {
    unsafe {
        let mut result_instr = EncodedX86_64Instruction::new();

        match gpr_or_memory {
            GPROrMemory::GPR { gpr } => {
                if bit16_requires_os_prefix && gpr.size().equals(Size::Bit16) {
                    result_instr.push_array([OPERAND_SIZE_OVERRIDE_PREFIX]);
                }

                let rex = {
                    const fn unwrap_rex(val: Option<NonZero<u8>>) -> u8 {
                        (if let Some(val) = val { val } else { REX }).get()
                    }
                    let mut rex_result = None;
                    if gpr.index_matches_bit8_high() && gpr.kind().equals(GPRKind::Bit8) {
                        rex_result = Some(REX);
                    }

                    if gpr.is_extended() {
                        let val = unwrap_rex(rex_result);
                        rex_result = Some(NonZero::new_unchecked(val | REX_B.get()));
                    }

                    if bit64_requires_rex_w && gpr.size().equals(Size::Bit64) {
                        let val = unwrap_rex(rex_result);
                        rex_result = Some(NonZero::new_unchecked(val | REX_W.get()));
                    }
                    rex_result
                };

                if let Some(rex) = rex {
                    result_instr.push_array([rex.get()]);
                }

                result_instr.push_array(opcode);
                result_instr.push_array([mod_rm(0b11, extended_opcode, gpr.lower_3_bits_index())]);
            }
            GPROrMemory::Memory { memory } => {
                let ext = memory.base_index_is_extended();
                let base_is_extended = ext.base_is_extended;
                let index_is_extended = ext.index_is_extended;

                if base_is_extended || index_is_extended {
                    let rex = rex(
                        0,
                        0,
                        if index_is_extended { 1 } else { 0 },
                        if base_is_extended { 1 } else { 0 },
                    );

                    result_instr.push_array([rex.get()]);
                }

                result_instr.push_array(opcode);

                let result = encode_memory(extended_opcode, memory);
                result_instr.push_slice(result.as_slice());
            }
        }

        result_instr
    }
}
