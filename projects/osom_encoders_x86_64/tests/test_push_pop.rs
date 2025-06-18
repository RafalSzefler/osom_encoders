#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

// PUSH tests
#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::AX }, &[0x66, 0xFF, 0xF0])]
fn test_push_rm16(#[case] rm16: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { push::encode_push_rm16(rm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0xFF, 0xF0])]
fn test_push_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { push::encode_push_rm64(rm64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPR::AX, &[0x66, 0x50])]
fn test_push_reg16(#[case] reg16: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { push::encode_push_reg16(reg16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPR::RAX, &[0x50])]
fn test_push_reg64(#[case] reg64: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { push::encode_push_reg64(reg64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(0x7F, &[0x6A, 0x7F])]
fn test_push_imm8(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = push::encode_push_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(0x1234, &[0x66, 0x68, 0x34, 0x12])]
fn test_push_imm16(#[case] imm16: i16, #[case] expected: &[u8]) {
    let imm16 = Immediate16::from_i16(imm16);
    let instr = push::encode_push_imm16(imm16);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(0x12345678, &[0x68, 0x78, 0x56, 0x34, 0x12])]
fn test_push_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = push::encode_push_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

// POP tests
#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::AX }, &[0x66, 0x8F, 0xC0])]
fn test_pop_rm16(#[case] rm16: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { pop::encode_pop_rm16(rm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0x8F, 0xC0])]
fn test_pop_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { pop::encode_pop_rm64(rm64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPR::AX, &[0x66, 0x58])]
fn test_pop_reg16(#[case] reg16: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { pop::encode_pop_reg16(reg16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPR::RAX, &[0x58])]
fn test_pop_reg64(#[case] reg64: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { pop::encode_pop_reg64(reg64) };
    assert_encoded_instruction_eq(expected, &instr);
}
