#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(-2, &[0x3C, 0xFE])]
#[case(-1, &[0x3C, 0xFF])]
#[case(0, &[0x3C, 0x00])]
#[case(1, &[0x3C, 0x01])]
#[case(10, &[0x3C, 0x0A])]
#[case(127, &[0x3C, 0x7F])]
fn test_cmp_AL_imm8(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = cmp::encode_cmp_AL_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-2, &[0x66, 0x3D, 0xFE, 0xFF])]
#[case(-1, &[0x66, 0x3D, 0xFF, 0xFF])]
#[case(0, &[0x66, 0x3D, 0x00, 0x00])]
#[case(1, &[0x66, 0x3D, 0x01, 0x00])]
#[case(10, &[0x66, 0x3D, 0x0A, 0x00])]
#[case(127, &[0x66, 0x3D, 0x7F, 0x00])]
#[case(1234, &[0x66, 0x3D, 0xD2, 0x04])]
fn test_cmp_AX_imm16(#[case] imm16: i16, #[case] expected: &[u8]) {
    let imm16 = Immediate16::from_i16(imm16);
    let instr = cmp::encode_cmp_AX_imm16(imm16);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-2, &[0x3D, 0xFE, 0xFF, 0xFF, 0xFF])]
#[case(-1, &[0x3D, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case(0, &[0x3D, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x3D, 0x01, 0x00, 0x00, 0x00])]
#[case(10, &[0x3D, 0x0A, 0x00, 0x00, 0x00])]
#[case(127, &[0x3D, 0x7F, 0x00, 0x00, 0x00])]
#[case(1234, &[0x3D, 0xD2, 0x04, 0x00, 0x00])]
#[case(12345678, &[0x3D, 0x4E, 0x61, 0xBC, 0x00])]
fn test_cmp_EAX_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = cmp::encode_cmp_EAX_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-2, &[0x48, 0x3D, 0xFE, 0xFF, 0xFF, 0xFF])]
#[case(-1, &[0x48, 0x3D, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case(0, &[0x48, 0x3D, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x48, 0x3D, 0x01, 0x00, 0x00, 0x00])]
#[case(10, &[0x48, 0x3D, 0x0A, 0x00, 0x00, 0x00])]
#[case(127, &[0x48, 0x3D, 0x7F, 0x00, 0x00, 0x00])]
#[case(1234, &[0x48, 0x3D, 0xD2, 0x04, 0x00, 0x00])]
#[case(12345678, &[0x48, 0x3D, 0x4E, 0x61, 0xBC, 0x00])]
fn test_cmp_RAX_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = cmp::encode_cmp_RAX_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AL }, Immediate8::from_i8(1), &[0x80, 0xF8, 0x01])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BL }, Immediate8::from_i8(5), &[0x80, 0xFB, 0x05])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AH }, Immediate8::from_i8(0), &[0x80, 0xFC, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::SPL }, Immediate8::from_i8(0), &[0x40, 0x80, 0xFC, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10B }, Immediate8::from_i8(-1), &[0x41, 0x80, 0xFA, 0xFF])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R15B }, Immediate8::from_i8(-15), &[0x41, 0x80, 0xFF, 0xF1])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, Immediate8::from_i8(1), &[0x80, 0x38, 0x01])]
fn test_cmp_rm8_imm8(#[case] gpr_or_memory: GPROrMemory, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm8_imm8(gpr_or_memory, imm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AX }, Immediate16::from_i16(1), &[0x66, 0x81, 0xF8, 0x01, 0x00])]
fn test_cmp_rm16_imm16(#[case] gpr_or_memory: GPROrMemory, #[case] imm16: Immediate16, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm16_imm16(gpr_or_memory, imm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EAX }, Immediate32::from_i32(1), &[0x81, 0xF8, 0x01, 0x00, 0x00, 0x00])]
fn test_cmp_rm32_imm32(#[case] gpr_or_memory: GPROrMemory, #[case] imm32: Immediate32, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm32_imm32(gpr_or_memory, imm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(1), &[0x48, 0x81, 0xF8, 0x01, 0x00, 0x00, 0x00])]
fn test_cmp_rm64_imm32(#[case] gpr_or_memory: GPROrMemory, #[case] imm32: Immediate32, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm64_imm32(gpr_or_memory, imm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AL }, GPR::BL, &[0x38, 0xD8])]
fn test_cmp_rm8_reg8(#[case] rm8: GPROrMemory, #[case] reg8: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm8_reg8(rm8, reg8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AX }, GPR::BX, &[0x66, 0x39, 0xD8])]
fn test_cmp_rm16_reg16(#[case] rm16: GPROrMemory, #[case] reg16: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm16_reg16(rm16, reg16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EAX }, GPR::EBX, &[0x39, 0xD8])]
fn test_cmp_rm32_reg32(#[case] rm32: GPROrMemory, #[case] reg32: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm32_reg32(rm32, reg32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RAX }, GPR::RBX, &[0x48, 0x39, 0xD8])]
fn test_cmp_rm64_reg64(#[case] rm64: GPROrMemory, #[case] reg64: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm64_reg64(rm64, reg64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::AL, GPROrMemory::GPR { gpr: GPR::BL }, &[0x3A, 0xC3])]
fn test_cmp_reg8_rm8(#[case] reg8: GPR, #[case] rm8: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_reg8_rm8(reg8, rm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::AX, GPROrMemory::GPR { gpr: GPR::BX }, &[0x66, 0x3B, 0xC3])]
fn test_cmp_reg16_rm16(#[case] reg16: GPR, #[case] rm16: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_reg16_rm16(reg16, rm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::EAX, GPROrMemory::GPR { gpr: GPR::EBX }, &[0x3B, 0xC3])]
fn test_cmp_reg32_rm32(#[case] reg32: GPR, #[case] rm32: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_reg32_rm32(reg32, rm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::RAX, GPROrMemory::GPR { gpr: GPR::RBX }, &[0x48, 0x3B, 0xC3])]
fn test_cmp_reg64_rm64(#[case] reg64: GPR, #[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_reg64_rm64(reg64, rm64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AX }, Immediate8::from_i8(1), &[0x66, 0x83, 0xF8, 0x01])]
fn test_cmp_rm16_imm8(#[case] gpr_or_memory: GPROrMemory, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm16_imm8(gpr_or_memory, imm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EAX }, Immediate8::from_i8(1), &[0x83, 0xF8, 0x01])]
fn test_cmp_rm32_imm8(#[case] gpr_or_memory: GPROrMemory, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm32_imm8(gpr_or_memory, imm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate8::from_i8(1), &[0x48, 0x83, 0xF8, 0x01])]
fn test_cmp_rm64_imm8(#[case] gpr_or_memory: GPROrMemory, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = unsafe { cmp::encode_cmp_rm64_imm8(gpr_or_memory, imm8) };
    assert_encoded_instruction_eq(expected, &instr);
}
