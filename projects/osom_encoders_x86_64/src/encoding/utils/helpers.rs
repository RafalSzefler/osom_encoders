use osom_encoders_common::osom_debug_assert;

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

pub const OPERAND_SIZE_OVERRIDE_PREFIX: u8 = 0x66;
pub const ADDRESS_SIZE_OVERRIDE_PREFIX: u8 = 0x67;
pub const REX: u8 = rex(0, 0, 0, 0);
pub const REX_B: u8 = rex(0, 0, 0, 1);
pub const REX_W: u8 = rex(1, 0, 0, 0);
