use osom_encoders_common::{FixedBuffer, osom_debug_assert};

use crate::models::{Displacement, GPR, Memory};

pub const OPERAND_SIZE_OVERRIDE_PREFIX: u8 = 0x66;
pub const ADDRESS_SIZE_OVERRIDE_PREFIX: u8 = 0x67;
pub const REX: u8 = rex(0, 0, 0, 0);
pub const REX_B: u8 = rex(0, 0, 0, 1);
pub const REX_W: u8 = rex(1, 0, 0, 0);

#[inline(always)]
pub const fn mod_rm(mod_field: u8, reg_field: u8, rm_field: u8) -> u8 {
    osom_debug_assert!(mod_field < 4);
    osom_debug_assert!(reg_field < 8);
    osom_debug_assert!(rm_field < 8);

    (mod_field << 6) | (reg_field << 3) | rm_field
}

#[inline(always)]
pub const fn sib(scale: u8, index: u8, base: u8) -> u8 {
    osom_debug_assert!(scale < 4);
    osom_debug_assert!(index < 8);
    osom_debug_assert!(base < 8);

    (scale << 6) | (index << 3) | base
}

#[inline(always)]
const fn rex(w: u8, r: u8, x: u8, b: u8) -> u8 {
    osom_debug_assert!(w < 2);
    osom_debug_assert!(r < 2);
    osom_debug_assert!(x < 2);
    osom_debug_assert!(b < 2);

    0b0100_0000 | (w << 3) | (r << 2) | (x << 1) | b
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodeMemoryResult {
    pub encoded_memory: FixedBuffer<5>,
    pub prefix: FixedBuffer<1>,
}

pub const fn encode_memory(reg_field: u8, memory: Memory) -> EncodeMemoryResult {
    osom_debug_assert!(memory.base().is_some() || memory.index().is_some());
    osom_debug_assert!(reg_field < 8);

    let mut buffer = FixedBuffer::new();
    let mut prefix = FixedBuffer::new();

    unsafe {
        if let Some(_index) = memory.index() {
            todo!()
        } else {
            let base = memory.base().unwrap_unchecked();
            match memory.displacement() {
                Displacement::None => {
                    // Handles `[REG]` kind of addressing.

                    if base.is_extended() {
                        prefix.push_array([REX_B]);
                    }

                    if base.equals(&GPR::RSP) || base.equals(&GPR::R12) {
                        // RSP and R12 require SIB byte. They can be efficiently encoded
                        // by using `mod == 0b00`, `rm_field == 0b100`, and `sib` with base `0b100`.
                        // Scale and index for `sib` is ignored.
                        buffer.push_array([mod_rm(0b00, reg_field, 0b100), sib(0b00, 0b00, 0b100)]);
                    } else if base.equals(&GPR::RBP) || base.equals(&GPR::R13) {
                        // RBP and R13 also require SIB byte. However they cannot use the same encoding
                        // as above and require `[base + disp8]` encoding with `disp8 == 0` (the last byte).
                        // This is achieved by using `mod == 0b01`, `rm_field == 0b100`, and `sib` with base `0b101`.
                        // Scale and index for `sib` is ignored.
                        buffer.push_array([mod_rm(0b01, reg_field, 0b100), sib(0b00, 0b00, 0b101), 0]);
                    } else {
                        buffer.push_array([mod_rm(0b00, reg_field, base.lower_3_bits_index())]);
                    }
                }
                Displacement::Imm8(_imm8) => {
                    todo!()
                }
                Displacement::Imm32(_imm32) => {
                    todo!()
                }
            }
        };
    }

    EncodeMemoryResult {
        encoded_memory: buffer,
        prefix,
    }
}
