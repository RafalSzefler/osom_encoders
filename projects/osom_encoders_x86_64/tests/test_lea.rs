#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(GPR::AX, Memory::Based { base: GPR::RDX, offset: Offset::from_i8(4) }, &[0x66, 0x8D, 0x42, 0x04])]
fn test_lea_reg16_m(#[case] reg16: GPR, #[case] m: Memory, #[case] expected: &[u8]) {
    let instr = unsafe { lea::encode_lea_reg16_m(reg16, m) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPR::EAX, Memory::Based { base: GPR::RDX, offset: Offset::from_i8(4) }, &[0x8D, 0x42, 0x04])]
fn test_lea_reg32_m(#[case] reg32: GPR, #[case] m: Memory, #[case] expected: &[u8]) {
    let instr = unsafe { lea::encode_lea_reg32_m(reg32, m) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPR::RAX, Memory::Based { base: GPR::RDX, offset: Offset::from_i8(4) }, &[0x48, 0x8D, 0x42, 0x04])]
#[case(GPR::R11, Memory::Based { base: GPR::RAX, offset: Offset::from_i8(-4) }, &[0x4C, 0x8D, 0x58, 0xFC])]
#[case(GPR::R11, Memory::Based { base: GPR::R12, offset: Offset::from_i8(-4) }, &[0x4D, 0x8D, 0x5C, 0x24, 0xFC])]
#[case(GPR::RDI, Memory::Based { base: GPR::R13, offset: Offset::from_i8(0) }, &[0x49, 0x8D, 0x7D, 0x00])]
#[case(GPR::R14, Memory::BasedScaled { base: GPR::R15, index: GPR::RAX, scale: Scale::Scale2, offset: Offset::from_i32(3) }, &[0x4D, 0x8D, 0xB4, 0x47, 0x03, 0x00, 0x00, 0x00])]
fn test_lea_reg64_m(#[case] reg64: GPR, #[case] m: Memory, #[case] expected: &[u8]) {
    let instr = unsafe { lea::encode_lea_reg64_m(reg64, m) };
    assert_encoded_instruction_eq(expected, &instr);
}
