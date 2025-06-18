#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(0x12345678, &[0xE8, 0x78, 0x56, 0x34, 0x12])]
fn test_call_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = call::encode_call_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0xFF, 0xD0])]
#[case(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RDX, offset: Offset::from_i8(2) } }, &[0xFF, 0x52, 0x02])]
fn test_call_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { call::encode_call_rm64(rm64) };
    assert_encoded_instruction_eq(expected, &instr);
}
