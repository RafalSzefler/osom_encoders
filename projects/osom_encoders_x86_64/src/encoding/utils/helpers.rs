use osom_encoders_common::{FixedBuffer, osom_debug_assert};

use crate::models::{GPR, Immediate32, Memory, Offset};

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
pub const fn rex(w: u8, r: u8, x: u8, b: u8) -> u8 {
    osom_debug_assert!(w < 2);
    osom_debug_assert!(r < 2);
    osom_debug_assert!(x < 2);
    osom_debug_assert!(b < 2);

    0b0100_0000 | (w << 3) | (r << 2) | (x << 1) | b
}

pub const fn encode_memory(reg_field: u8, memory: &Memory) -> FixedBuffer<15> {
    osom_debug_assert!(reg_field < 8);

    match memory {
        Memory::Based { base, offset } => encoded_based_memory(reg_field, *base, *offset),
        Memory::Scaled { .. } => {
            todo!()
        }
        Memory::BasedScaled { .. } => {
            todo!()
        }
        Memory::RelativeToRIP { offset } => unsafe {
            let mut buffer = FixedBuffer::new();
            buffer.push_array([mod_rm(0b00, reg_field, 0b101)]);

            // RIP-relative addressing requires 32-bit displacement, always.
            let offset = match offset {
                Offset::None => Immediate32::from_u32(0),
                Offset::Bit8(imm8) => Immediate32::from_u32(imm8.value() as u32),
                Offset::Bit32(imm32) => *imm32,
            };
            buffer.push_array(offset.encode());

            buffer
        },
    }
}

const fn encoded_based_memory(reg_field: u8, base: GPR, offset: Offset) -> FixedBuffer<15> {
    let mut buffer = FixedBuffer::new();

    unsafe {
        match offset {
            Offset::None => {
                if base.equals(&GPR::RSP) || base.equals(&GPR::R12) {
                    // RSP and R12 require SIB byte. They can be efficiently encoded
                    // by using `mod == 0b00`, `rm_field == 0b100`, and `sib` with base `0b100`
                    // and index `0b100`. The scale is ignored.
                    buffer.push_array([mod_rm(0b00, reg_field, 0b100), sib(0b00, 0b100, 0b100)]);
                } else if base.equals(&GPR::RBP) || base.equals(&GPR::R13) {
                    // RBP and R13 can be efficiently encoded through mod_rm with `mod == 0b01`,
                    // which corresponds to `[r/m + disp8]` encoding. The `rm_field` becomes
                    // `0b101` which corresponds to RBP and R13 lower 3 bit indexes.
                    buffer.push_array([mod_rm(0b01, reg_field, 0b101), 0]);
                } else {
                    buffer.push_array([mod_rm(0b00, reg_field, base.lower_3_bits_index())]);
                }
            }
            Offset::Bit8(_imm8) => {
                todo!()
            }
            Offset::Bit32(_imm32) => {
                todo!()
            }
        }
    }

    buffer
}
