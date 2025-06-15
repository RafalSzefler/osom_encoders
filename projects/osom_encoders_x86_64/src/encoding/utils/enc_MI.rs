use osom_encoders_common::osom_debug_assert;

use crate::{
    encoding::utils::helpers::{REX, REX_B, encode_memory, mod_rm, rex},
    models::{EncodedX86_64Instruction, GPRKind, GPROrMemory, Immediate8, Size},
};

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
    gpr_or_memory: GPROrMemory,
    imm8: Immediate8,
) -> EncodedX86_64Instruction {
    unsafe {
        let mut instr = EncodedX86_64Instruction::new();

        match &gpr_or_memory {
            GPROrMemory::GPR { gpr } => {
                osom_debug_assert!(gpr.size().equals(Size::Bit8));

                if gpr.is_extended() {
                    instr.push_array([REX_B]);
                } else if gpr.index_matches_bit8_high() && gpr.kind().equals(GPRKind::Bit8) {
                    instr.push_array([REX]);
                }
                instr.push_array(opcode);
                instr.push_array([mod_rm(0b11, extended_opcode, gpr.lower_3_bits_index())]);
                instr.push_array(imm8.encode());
            }
            GPROrMemory::Memory { memory } => {
                let extended_base = memory.extended_base();
                let extended_index = memory.extended_index();

                if extended_base || extended_index {
                    let rex = rex(
                        0,
                        0,
                        if extended_index { 1 } else { 0 },
                        if extended_base { 1 } else { 0 },
                    );

                    instr.push_array([rex]);
                }

                instr.push_array(opcode);

                let result = encode_memory(extended_opcode, memory);
                instr.push_slice(result.as_slice());
                instr.push_array(imm8.encode());
            }
        }

        instr
    }
}
