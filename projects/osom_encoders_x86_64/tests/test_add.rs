#![allow(non_snake_case)]
mod common;

use rstest::rstest;

use osom_encoders_x86_64::encoding::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(-2, &[0x04, 0xFE])]
#[case(-1, &[0x04, 0xFF])]
#[case(0, &[0x04, 0x00])]
#[case(1, &[0x04, 0x01])]
#[case(10, &[0x04, 0x0A])]
#[case(127, &[0x04, 0x7F])]
fn test_add_AL_imm8(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = enc::encode_add_AL_imm8(imm8);
    common::assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-2, &[0x66, 0x05, 0xFE, 0xFF])]
#[case(-1, &[0x66, 0x05, 0xFF, 0xFF])]
#[case(0, &[0x66, 0x05, 0x00, 0x00])]
#[case(1, &[0x66, 0x05, 0x01, 0x00])]
#[case(10, &[0x66, 0x05, 0x0A, 0x00])]
#[case(127, &[0x66, 0x05, 0x7F, 0x00])]
#[case(1234, &[0x66, 0x05, 0xD2, 0x04])]
fn test_add_AX_imm16(#[case] imm16: i16, #[case] expected: &[u8]) {
    let imm16 = Immediate16::from_i16(imm16);
    let instr = enc::encode_add_AX_imm16(imm16);
    common::assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-2, &[0x05, 0xFE, 0xFF, 0xFF, 0xFF])]
#[case(-1, &[0x05, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case(0, &[0x05, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x05, 0x01, 0x00, 0x00, 0x00])]
#[case(10, &[0x05, 0x0A, 0x00, 0x00, 0x00])]
#[case(127, &[0x05, 0x7F, 0x00, 0x00, 0x00])]
#[case(1234, &[0x05, 0xD2, 0x04, 0x00, 0x00])]
#[case(12345678, &[0x05, 0x4E, 0x61, 0xBC, 0x00])]
fn test_add_EAX_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = enc::encode_add_EAX_imm32(imm32);
    common::assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-2, &[0x48, 0x05, 0xFE, 0xFF, 0xFF, 0xFF])]
#[case(-1, &[0x48, 0x05, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case(0, &[0x48, 0x05, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x48, 0x05, 0x01, 0x00, 0x00, 0x00])]
#[case(10, &[0x48, 0x05, 0x0A, 0x00, 0x00, 0x00])]
#[case(127, &[0x48, 0x05, 0x7F, 0x00, 0x00, 0x00])]
#[case(1234, &[0x48, 0x05, 0xD2, 0x04, 0x00, 0x00])]
#[case(12345678, &[0x48, 0x05, 0x4E, 0x61, 0xBC, 0x00])]
fn test_add_RAX_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = enc::encode_add_RAX_imm32(imm32);
    common::assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AL }, Immediate8::from_i8(1), &[0x80, 0xC0, 0x01])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BL }, Immediate8::from_i8(5), &[0x80, 0xC3, 0x05])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AH }, Immediate8::from_i8(0), &[0x80, 0xC4, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::SPL }, Immediate8::from_i8(0), &[0x40, 0x80, 0xC4, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10B }, Immediate8::from_i8(-1), &[0x41, 0x80, 0xC2, 0xFF])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R15B }, Immediate8::from_i8(-15), &[0x41, 0x80, 0xC7, 0xF1])]
fn test_add_rm8_imm8(#[case] gpr_or_memory: GPROrMemory, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = enc::encode_add_rm8_imm8(gpr_or_memory, imm8);
    common::assert_encoded_instruction_eq(expected, &instr);
}
