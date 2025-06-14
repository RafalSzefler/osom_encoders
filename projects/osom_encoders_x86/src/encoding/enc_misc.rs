use osom_encoders_common::osom_assert;

use crate::models::{EncodedX86Instruction, Immediate16};

#[inline(always)]
pub const fn encode_lock_prefix() -> EncodedX86Instruction {
    unsafe { EncodedX86Instruction::from_array([0xF0]) }
}

#[inline(always)]
pub const fn encode_ret() -> EncodedX86Instruction {
    unsafe { EncodedX86Instruction::from_array([0xC3]) }
}

#[inline(always)]
pub const fn encode_ret_imm16(imm16: Immediate16) -> EncodedX86Instruction {
    unsafe {
        let mut instruction = EncodedX86Instruction::from_array([0xC2]);
        instruction.push_array(imm16.encode());
        instruction
    }
}

#[inline(always)]
pub const fn encode_nop() -> EncodedX86Instruction {
    unsafe { EncodedX86Instruction::from_array([0x90]) }
}

/// # Safety
///
/// This function is unsafe because it doesn't validate the byte length.
/// It is up to caller to ensure that `byte_length` is in range 1..=9.
/// Otherwise the behaviour is undefined in release mode. While it
/// panics in debug mode.
pub const unsafe fn encode_nop_with_length(byte_length: u8) -> EncodedX86Instruction {
    osom_assert!(byte_length >= 1 && byte_length <= 9);

    unsafe {
        let mut instruction = EncodedX86Instruction::new();

        match byte_length {
            1 => instruction.push_array([0x90]),
            2 => instruction.push_array([0x66, 0x90]),
            3 => instruction.push_array([0x0F, 0x1F, 0x00]),
            4 => instruction.push_array([0x0F, 0x1F, 0x40, 0x00]),
            5 => instruction.push_array([0x0F, 0x1F, 0x44, 0x00, 0x00]),
            6 => instruction.push_array([0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00]),
            7 => instruction.push_array([0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00]),
            8 => instruction.push_array([0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
            9 => instruction.push_array([0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
            _ => core::hint::unreachable_unchecked(),
        }

        instruction
    }
}
