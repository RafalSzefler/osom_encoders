use osom_encoders_common::osom_assert;

use crate::{encoding::utils::helpers::{mod_rm, REX, REX_B}, models::{EncodedX86Instruction, GPRKind, GPROrMemory, Immediate8, Size}};

/// # Safety
/// 
/// This function is unsafe because:
/// * It doesn't validate the `opcode` and `extended_opcode`
/// * It doesn't validate `gpr_or_memory`, in particular:
/// * * `gpr` has to be a 8-bit register
/// * * `memory` has to be a valid memory operand, meaning 32-bit or 64-bit depending on the architecture
///     (even though the `gpr_or_memory` is 8-bit as a whole, this applies to `gpr` only, not `memory`)
/// 
/// It is up to the caller to ensure that these are valid.
pub const unsafe fn encode_MI_rm8_imm8<const T: usize>(opcode: [u8; T], extended_opcode: u8, gpr_or_memory: GPROrMemory, imm8: Immediate8) -> EncodedX86Instruction {
    unsafe {
        let mut instr = EncodedX86Instruction::new();

        match gpr_or_memory {
            GPROrMemory::GPR { gpr } => {
                osom_assert!(gpr.size().equals(Size::Bit8));

                if gpr.is_extended() {
                    instr.push_array([REX_B]);
                } else if gpr.index_matches_bit8_high() && gpr.kind().equals(GPRKind::Bit8) {
                    instr.push_array([REX]);
                }
                instr.push_array(opcode);
                instr.push_array([
                    mod_rm(0b11, extended_opcode, gpr.index().as_u8())
                ]);
                instr.push_array(imm8.encode());
            },
            GPROrMemory::Memory { memory } => {
                todo!()
            },
        }

        instr
    }
}
