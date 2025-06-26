use core::num::NonZero;

use osom_encoders_common::{FixedBuffer, osom_debug_assert};

use crate::models::{GPR, Memory, Offset, Scale};

pub const OPERAND_SIZE_OVERRIDE_PREFIX: u8 = 0x66;
pub const REX: NonZero<u8> = rex(0, 0, 0, 0);
pub const REX_W: NonZero<u8> = rex(1, 0, 0, 0);
pub const REX_R: NonZero<u8> = rex(0, 1, 0, 0);
pub const REX_B: NonZero<u8> = rex(0, 0, 0, 1);

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
pub const fn rex(w: u8, r: u8, x: u8, b: u8) -> NonZero<u8> {
    osom_debug_assert!(w < 2);
    osom_debug_assert!(r < 2);
    osom_debug_assert!(x < 2);
    osom_debug_assert!(b < 2);

    unsafe { NonZero::new_unchecked(0b0100_0000 | (w << 3) | (r << 2) | (x << 1) | b) }
}

pub type MemoryBuffer = FixedBuffer<7>;

pub const fn encode_memory(reg_field: u8, memory: &Memory) -> MemoryBuffer {
    osom_debug_assert!(reg_field < 8);
    osom_debug_assert!(memory_contains_bit64_registers(memory));

    match memory {
        Memory::Based { base, offset } => encode_based_memory(reg_field, *base, *offset),
        Memory::Scaled { index, scale, offset } => encode_scaled_memory(reg_field, *index, *scale, *offset),
        Memory::BasedScaled {
            base,
            index,
            scale,
            offset,
        } => encode_based_scaled_memory(reg_field, *base, *index, *scale, *offset),
        Memory::RelativeToRIP { offset } => unsafe {
            let mut buffer = MemoryBuffer::new();
            buffer.push_array([mod_rm(0b00, reg_field, 0b101)]);

            // RIP-relative addressing requires 32-bit displacement, always.
            let offset = offset.as_sign_extended_imm32();
            buffer.push_array(offset.encode());

            buffer
        },
    }
}

const fn encode_based_memory(reg_field: u8, base: GPR, offset: Offset) -> MemoryBuffer {
    let mut buffer = MemoryBuffer::new();

    unsafe {
        match offset {
            Offset::None => {
                if base.equals(&GPR::RSP) || base.equals(&GPR::R12) {
                    // RSP and R12 require SIB byte. They can be efficiently encoded
                    // by using `mod == 0b00`, `rm_field == 0b100`, and `sib` with base `0b100`
                    // and index `0b100`. The scale is ignored.
                    buffer.push_array([mod_rm(0b00, reg_field, 0b100), sib(0b00, 0b100, 0b100)]);
                } else if base.equals(&GPR::RBP) || base.equals(&GPR::R13) {
                    // Normal `mod_field == 0b00` encoding for those registers
                    // is actually RIP-relative addressing, which is not what we want.
                    //
                    // And so RBP and R13 has to be encoded differently.
                    // The best solution is through `mod_field == 0b01`,
                    // which corresponds to `[r/m + disp8]` encoding. The `rm_field` becomes
                    // `0b101` which corresponds to RBP and R13 lower 3 bit indexes.
                    // Finally we add a fixed zero byte to indicate that the displacement is 0.
                    buffer.push_array([mod_rm(0b01, reg_field, 0b101), 0]);
                } else {
                    // Normal encoding. Combinations of offset and registers above
                    // should be avoided (by the user) in favor of this one.
                    buffer.push_array([mod_rm(0b00, reg_field, base.lower_3_bits_index())]);
                }
            }
            Offset::Bit8(imm8) => {
                if base.equals(&GPR::RSP) || base.equals(&GPR::R12) {
                    // Similarly like before: requires SIB byte, but with
                    // different `mod_field`.
                    buffer.push_array([mod_rm(0b01, reg_field, 0b100), sib(0b00, 0b100, 0b100)]);
                } else {
                    // Unlike previous case, RBP and R13 is included for 8-bit offset.
                    buffer.push_array([mod_rm(0b01, reg_field, base.lower_3_bits_index())]);
                }
                buffer.push_array(imm8.encode());
            }
            Offset::Bit32(imm32) => {
                // Analogous to imm8 case, but with different `mod_field`.
                if base.equals(&GPR::RSP) || base.equals(&GPR::R12) {
                    buffer.push_array([mod_rm(0b10, reg_field, 0b100), sib(0b00, 0b100, 0b100)]);
                } else {
                    buffer.push_array([mod_rm(0b10, reg_field, base.lower_3_bits_index())]);
                }
                buffer.push_array(imm32.encode());
            }
        }
    }

    buffer
}

const fn encode_scaled_memory(reg_field: u8, index: GPR, scale: Scale, offset: Offset) -> MemoryBuffer {
    let mut buffer = MemoryBuffer::new();
    unsafe {
        // All of these require SIB byte.
        // Note that `index == RSP` is not allowed in SIB.
        osom_debug_assert!(index.index() != GPR::RSP.index());

        // The only encoding that allows index without base is `mod_field == 0b00`
        // and SIB base `0b100` (the reason why RSP is not allowed btw).
        // This encoding requires mandatory disp32 at the end, so we need
        // to convert the offset.
        buffer.push_array([
            mod_rm(0b00, reg_field, 0b100),
            sib(scale.index(), index.lower_3_bits_index(), 0b101),
        ]);

        let offset = offset.as_sign_extended_imm32();
        buffer.push_array(offset.encode());
    }
    buffer
}

const fn encode_based_scaled_memory(
    reg_field: u8,
    base: GPR,
    index: GPR,
    scale: Scale,
    offset: Offset,
) -> MemoryBuffer {
    let mut buffer = MemoryBuffer::new();
    unsafe {
        // All of these require SIB byte, i.e. `rm_field == 0b100` in mod_rm byte.
        // Note that `index == RSP` is not allowed in SIB.
        osom_debug_assert!(index.index() != GPR::RSP.index());

        // These can be encoded in multiple ways. The most efficient encoding
        // depends on the offset.

        let sib_byte = sib(scale.index(), index.lower_3_bits_index(), base.lower_3_bits_index());

        if matches!(offset, Offset::None) && (base.equals(&GPR::RBP) || base.equals(&GPR::R13)) {
            // Special case: these registers require `mod_field == 0b01`,
            // meaning `[base + index * scale + disp8]` encoding. The disp8 will
            // be fixed at 0.
            buffer.push_array([mod_rm(0b01, reg_field, 0b100), sib_byte, 0]);
            return buffer;
        }

        // The rest follows the same pattern, they differ in `mod_field` and immediate size only.

        let mod_field = match offset {
            Offset::None => 0b00,
            Offset::Bit8(_) => 0b01,
            Offset::Bit32(_) => 0b10,
        };

        buffer.push_array([mod_rm(mod_field, reg_field, 0b100), sib_byte]);

        match offset {
            Offset::None => {}
            Offset::Bit8(imm8) => {
                buffer.push_array(imm8.encode());
            }
            Offset::Bit32(imm32) => {
                buffer.push_array(imm32.encode());
            }
        }
    }
    buffer
}

#[cfg(debug_assertions)]
#[must_use]
const fn memory_contains_bit64_registers(memory: &Memory) -> bool {
    use crate::models::Size;
    match memory {
        Memory::Based { base, .. } => base.size().equals(Size::Bit64),
        Memory::Scaled { index, .. } => index.size().equals(Size::Bit64),
        Memory::BasedScaled { base, index, .. } => base.size().equals(Size::Bit64) && index.size().equals(Size::Bit64),
        Memory::RelativeToRIP { .. } => true,
    }
}
