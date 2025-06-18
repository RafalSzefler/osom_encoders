#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AL }, Immediate8::from_i8(1), &[0xC6, 0xC0, 0x01])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BL }, Immediate8::from_i8(5), &[0xC6, 0xC3, 0x05])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AH }, Immediate8::from_i8(0), &[0xC6, 0xC4, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::SPL }, Immediate8::from_i8(0), &[0x40, 0xC6, 0xC4, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10B }, Immediate8::from_i8(-1), &[0x41, 0xC6, 0xC2, 0xFF])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R15B }, Immediate8::from_i8(-15), &[0x41, 0xC6, 0xC7, 0xF1])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RSP, offset: Offset::None } }, Immediate8::from_i8(3), &[0xC6, 0x04, 0x24, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R12, offset: Offset::None } }, Immediate8::from_i8(3), &[0x41, 0xC6, 0x04, 0x24, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBP, offset: Offset::None } }, Immediate8::from_i8(4), &[0xC6, 0x45, 0x00, 0x04])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R13, offset: Offset::None } }, Immediate8::from_i8(4), &[0x41, 0xC6, 0x45, 0x00, 0x04])]
#[case::memory(GPROrMemory::Memory { memory: Memory::RelativeToRIP { offset: Offset::None } }, Immediate8::from_i8(1), &[0xC6, 0x05, 0x00, 0x00, 0x00, 0x00, 0x01])]
#[case::memory(GPROrMemory::Memory { memory: Memory::RelativeToRIP { offset: Offset::Bit8(Immediate8::from_i8(-2)) } }, Immediate8::from_i8(3), &[0xC6, 0x05, 0xFE, 0xFF, 0xFF, 0xFF, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::RelativeToRIP { offset: Offset::Bit8(Immediate8::from_i8(1)) } }, Immediate8::from_i8(-1), &[0xC6, 0x05, 0x01, 0x00, 0x00, 0x00, 0xFF])]
#[case::memory(GPROrMemory::Memory { memory: Memory::RelativeToRIP { offset: Offset::Bit32(Immediate32::from_i32(-2000)) } }, Immediate8::from_i8(3), &[0xC6, 0x05, 0x30, 0xF8, 0xFF, 0xFF, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::from_i8(4) } }, Immediate8::from_i8(3), &[0xC6, 0x40, 0x04, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RSP, offset: Offset::from_i8(-1) } }, Immediate8::from_i8(3), &[0xC6, 0x44, 0x24, 0xFF, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBP, offset: Offset::from_i8(0) } }, Immediate8::from_i8(3), &[0xC6, 0x45, 0x00, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R15, offset: Offset::from_i8(7) } }, Immediate8::from_i8(-5), &[0x41, 0xC6, 0x47, 0x07, 0xFB])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::from_i32(4) } }, Immediate8::from_i8(3), &[0xC6, 0x80, 0x04, 0x00, 0x00, 0x00, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RSP, offset: Offset::from_i32(-1) } }, Immediate8::from_i8(3), &[0xC6, 0x84, 0x24, 0xFF, 0xFF, 0xFF, 0xFF, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBP, offset: Offset::from_i32(0) } }, Immediate8::from_i8(3), &[0xC6, 0x85, 0x00, 0x00, 0x00, 0x00, 0x03])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R15, offset: Offset::from_i32(7) } }, Immediate8::from_i8(-5), &[0x41, 0xC6, 0x87, 0x07, 0x00, 0x00, 0x00, 0xFB])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Scaled { index: GPR::RAX, scale: Scale::Scale2, offset: Offset::from_i32(7) } }, Immediate8::from_i8(-5), &[0xC6, 0x04, 0x45, 0x07, 0x00, 0x00, 0x00, 0xFB])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Scaled { index: GPR::RBP, scale: Scale::Scale4, offset: Offset::from_i32(-1) } }, Immediate8::from_i8(6), &[0xC6, 0x04, 0xAD, 0xFF, 0xFF, 0xFF, 0xFF, 0x06])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Scaled { index: GPR::R14, scale: Scale::Scale4, offset: Offset::from_i32(-1000) } }, Immediate8::from_i8(120), &[0x42, 0xC6, 0x04, 0xB5, 0x18, 0xFC, 0xFF, 0xFF, 0x78])]
#[case::memory(GPROrMemory::Memory { memory: Memory::BasedScaled { base: GPR::RAX, index: GPR::R14, scale: Scale::Scale2, offset: Offset::None } }, Immediate8::from_i8(1), &[0x42, 0xC6, 0x04, 0x70, 0x01])]
#[case::memory(GPROrMemory::Memory { memory: Memory::BasedScaled { base: GPR::R13, index: GPR::R14, scale: Scale::Scale2, offset: Offset::None } }, Immediate8::from_i8(1), &[0x43, 0xC6, 0x44, 0x75, 0x00, 0x01])]
#[case::memory(GPROrMemory::Memory { memory: Memory::BasedScaled { base: GPR::RAX, index: GPR::RBX, scale: Scale::Scale4, offset: Offset::from_i8(1) } }, Immediate8::from_i8(-1), &[0xC6, 0x44, 0x98, 0x01, 0xFF])]
#[case::memory(GPROrMemory::Memory { memory: Memory::BasedScaled { base: GPR::RAX, index: GPR::RBX, scale: Scale::Scale4, offset: Offset::from_i8(-1) } }, Immediate8::from_i8(1), &[0xC6, 0x44, 0x98, 0xFF, 0x01])]
fn test_mov_rm8_imm8(#[case] gpr_or_memory: GPROrMemory, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm8_imm8(gpr_or_memory, imm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::AX }, Immediate16::from_i16(1), &[0x66, 0xC7, 0xC0, 0x01, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BX }, Immediate16::from_i16(5), &[0x66, 0xC7, 0xC3, 0x05, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10W }, Immediate16::from_i16(-1), &[0x66, 0x41, 0xC7, 0xC2, 0xFF, 0xFF])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R15W }, Immediate16::from_i16(-15), &[0x66, 0x41, 0xC7, 0xC7, 0xF1, 0xFF])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RSP, offset: Offset::None } }, Immediate16::from_i16(3), &[0x66, 0xC7, 0x04, 0x24, 0x03, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R12, offset: Offset::None } }, Immediate16::from_i16(3), &[0x66, 0x41, 0xC7, 0x04, 0x24, 0x03, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBP, offset: Offset::None } }, Immediate16::from_i16(4), &[0x66, 0xC7, 0x45, 0x00, 0x04, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R13, offset: Offset::None } }, Immediate16::from_i16(4), &[0x66, 0x41, 0xC7, 0x45, 0x00, 0x04, 0x00])]
fn test_mov_rm16_imm16(#[case] gpr_or_memory: GPROrMemory, #[case] imm16: Immediate16, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm16_imm16(gpr_or_memory, imm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EAX }, Immediate32::from_i32(1), &[0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EBX }, Immediate32::from_i32(5), &[0xC7, 0xC3, 0x05, 0x00, 0x00, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10D }, Immediate32::from_i32(-1), &[0x41, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R15D }, Immediate32::from_i32(-15), &[0x41, 0xC7, 0xC7, 0xF1, 0xFF, 0xFF, 0xFF])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RSP, offset: Offset::None } }, Immediate32::from_i32(3), &[0xC7, 0x04, 0x24, 0x03, 0x00, 0x00, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R12, offset: Offset::None } }, Immediate32::from_i32(3), &[0x41, 0xC7, 0x04, 0x24, 0x03, 0x00, 0x00, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBP, offset: Offset::None } }, Immediate32::from_i32(4), &[0xC7, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R13, offset: Offset::None } }, Immediate32::from_i32(4), &[0x41, 0xC7, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00])]
fn test_mov_rm32_imm32(#[case] gpr_or_memory: GPROrMemory, #[case] imm32: Immediate32, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm32_imm32(gpr_or_memory, imm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RAX }, Immediate32::from_i32(1), &[0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RBX }, Immediate32::from_i32(5), &[0x48, 0xC7, 0xC3, 0x05, 0x00, 0x00, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10 }, Immediate32::from_i32(-1), &[0x49, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R15 }, Immediate32::from_i32(-15), &[0x49, 0xC7, 0xC7, 0xF1, 0xFF, 0xFF, 0xFF])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RSP, offset: Offset::None } }, Immediate32::from_i32(3), &[0xC7, 0x04, 0x24, 0x03, 0x00, 0x00, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R12, offset: Offset::None } }, Immediate32::from_i32(3), &[0x41, 0xC7, 0x04, 0x24, 0x03, 0x00, 0x00, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBP, offset: Offset::None } }, Immediate32::from_i32(4), &[0xC7, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00])]
#[case::memory(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R13, offset: Offset::None } }, Immediate32::from_i32(4), &[0x41, 0xC7, 0x45, 0x00, 0x04, 0x00, 0x00, 0x00])]
fn test_mov_rm64_imm32(#[case] gpr_or_memory: GPROrMemory, #[case] imm32: Immediate32, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm64_imm32(gpr_or_memory, imm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::al(GPR::AL, Immediate8::from_i8(1), &[0xB0, 0x01])]
#[case::bl(GPR::BL, Immediate8::from_i8(5), &[0xB3, 0x05])]
#[case::ah(GPR::AH, Immediate8::from_i8(0), &[0xB4, 0x00])]
#[case::spl(GPR::SPL, Immediate8::from_i8(0), &[0x40, 0xB4, 0x00])]
#[case::r10b(GPR::R10B, Immediate8::from_i8(-1), &[0x41, 0xB2, 0xFF])]
#[case::r15b(GPR::R15B, Immediate8::from_i8(-15), &[0x41, 0xB7, 0xF1])]
fn test_mov_reg8_imm8(#[case] reg8: GPR, #[case] imm8: Immediate8, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg8_imm8(reg8, imm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::ax(GPR::AX, Immediate16::from_i16(1), &[0x66, 0xB8, 0x01, 0x00])]
#[case::bx(GPR::BX, Immediate16::from_i16(5), &[0x66, 0xBB, 0x05, 0x00])]
#[case::r10w(GPR::R10W, Immediate16::from_i16(-1), &[0x66, 0x41, 0xBA, 0xFF, 0xFF])]
#[case::r15w(GPR::R15W, Immediate16::from_i16(-15), &[0x66, 0x41, 0xBF, 0xF1, 0xFF])]
fn test_mov_reg16_imm16(#[case] reg16: GPR, #[case] imm16: Immediate16, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg16_imm16(reg16, imm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::eax(GPR::EAX, Immediate32::from_i32(1), &[0xB8, 0x01, 0x00, 0x00, 0x00])]
#[case::ebx(GPR::EBX, Immediate32::from_i32(5), &[0xBB, 0x05, 0x00, 0x00, 0x00])]
#[case::r10d(GPR::R10D, Immediate32::from_i32(-1), &[0x41, 0xBA, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case::r15d(GPR::R15D, Immediate32::from_i32(-15), &[0x41, 0xBF, 0xF1, 0xFF, 0xFF, 0xFF])]
fn test_mov_reg32_imm32(#[case] reg32: GPR, #[case] imm32: Immediate32, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg32_imm32(reg32, imm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::rax(GPR::RAX, Immediate64::from_i64(1), &[0x48, 0xB8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
#[case::rbx(GPR::RBX, Immediate64::from_i64(5), &[0x48, 0xBB, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
#[case::r10(GPR::R10, Immediate64::from_i64(-1), &[0x49, 0xBA, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case::r15(GPR::R15, Immediate64::from_i64(-15), &[0x49, 0xBF, 0xF1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])]
fn test_mov_reg64_imm64(#[case] reg64: GPR, #[case] imm64: Immediate64, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg64_imm64(reg64, imm64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::AL, GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, &[0x8A, 0x00])]
#[case::gpr(GPR::AL, GPROrMemory::GPR { gpr: GPR::BL }, &[0x8A, 0xC3])]
#[case::gpr(GPR::AL, GPROrMemory::GPR { gpr: GPR::BH }, &[0x8A, 0xC7])]
#[case::gpr(GPR::BL, GPROrMemory::GPR { gpr: GPR::R10B }, &[0x41, 0x8A, 0xDA])]
#[case::gpr(GPR::R11B, GPROrMemory::GPR { gpr: GPR::DL }, &[0x44, 0x8A, 0xDA,])]
#[case::gpr(GPR::R11B, GPROrMemory::GPR { gpr: GPR::R12B }, &[0x45, 0x8A, 0xDC])]
fn test_mov_reg8_rm8(#[case] reg8: GPR, #[case] rm8: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg8_rm8(reg8, rm8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::AX, GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, &[0x66, 0x8B, 0x00])]
#[case::gpr(GPR::AX, GPROrMemory::GPR { gpr: GPR::BX }, &[0x66, 0x8B, 0xC3])]
#[case::gpr(GPR::AX, GPROrMemory::GPR { gpr: GPR::BX }, &[0x66, 0x8B, 0xC3])]
#[case::gpr(GPR::BX, GPROrMemory::GPR { gpr: GPR::R10W }, &[0x66, 0x41, 0x8B, 0xDA])]
#[case::gpr(GPR::R11W, GPROrMemory::GPR { gpr: GPR::DX }, &[0x66, 0x44, 0x8B, 0xDA])]
#[case::gpr(GPR::R11W, GPROrMemory::GPR { gpr: GPR::R12W }, &[0x66, 0x45, 0x8B, 0xDC])]
fn test_mov_reg16_rm16(#[case] reg16: GPR, #[case] rm16: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg16_rm16(reg16, rm16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::EAX, GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, &[0x8B, 0x00])]
#[case::gpr(GPR::EAX, GPROrMemory::GPR { gpr: GPR::EBX }, &[0x8B, 0xC3])]
#[case::gpr(GPR::EAX, GPROrMemory::GPR { gpr: GPR::EBX }, &[0x8B, 0xC3])]
#[case::gpr(GPR::EBX, GPROrMemory::GPR { gpr: GPR::R10D }, &[0x41, 0x8B, 0xDA])]
#[case::gpr(GPR::R11D, GPROrMemory::GPR { gpr: GPR::EDX }, &[0x44, 0x8B, 0xDA])]
#[case::gpr(GPR::R11D, GPROrMemory::GPR { gpr: GPR::R12D }, &[0x45, 0x8B, 0xDC])]
fn test_mov_reg32_rm32(#[case] reg32: GPR, #[case] rm32: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg32_rm32(reg32, rm32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPR::RAX, GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, &[0x48, 0x8B, 0x00])]
#[case::gpr(GPR::RAX, GPROrMemory::GPR { gpr: GPR::RBX }, &[0x48, 0x8B, 0xC3])]
#[case::gpr(GPR::RAX, GPROrMemory::GPR { gpr: GPR::RBX }, &[0x48, 0x8B, 0xC3])]
#[case::gpr(GPR::RBX, GPROrMemory::GPR { gpr: GPR::R10 }, &[0x49, 0x8B, 0xDA])]
#[case::gpr(GPR::R11, GPROrMemory::GPR { gpr: GPR::RDX }, &[0x4C, 0x8B, 0xDA])]
#[case::gpr(GPR::R11, GPROrMemory::GPR { gpr: GPR::R12 }, &[0x4D, 0x8B, 0xDC])]
fn test_mov_reg64_rm64(#[case] reg64: GPR, #[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_reg64_rm64(reg64, rm64) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, GPR::AL, &[0x88, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BL }, GPR::AL, &[0x88, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BH }, GPR::AL, &[0x88, 0xC7])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10B }, GPR::BL, &[0x41, 0x88, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::DL }, GPR::R11B, &[0x44, 0x88, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R12B }, GPR::R11B, &[0x45, 0x88, 0xDC])]
fn test_mov_rm8_reg8(#[case] rm8: GPROrMemory, #[case] reg8: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm8_reg8(rm8, reg8) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, GPR::AX, &[0x66, 0x89, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BX }, GPR::AX, &[0x66, 0x89, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::BX }, GPR::AX, &[0x66, 0x89, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10W }, GPR::BX, &[0x66, 0x41, 0x89, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::DX }, GPR::R11W, &[0x66, 0x44, 0x89, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R12W }, GPR::R11W, &[0x66, 0x45, 0x89, 0xDC])]
fn test_mov_rm16_reg16(#[case] rm16: GPROrMemory, #[case] reg16: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm16_reg16(rm16, reg16) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, GPR::EAX, &[0x89, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EBX }, GPR::EAX, &[0x89, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EBX }, GPR::EAX, &[0x89, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10D }, GPR::EBX, &[0x41, 0x89, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::EDX }, GPR::R11D, &[0x44, 0x89, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R12D }, GPR::R11D, &[0x45, 0x89, 0xDC])]
fn test_mov_rm32_reg32(#[case] rm32: GPROrMemory, #[case] reg32: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm32_reg32(rm32, reg32) };
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case::gpr(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RAX, offset: Offset::None } }, GPR::RAX, &[0x48, 0x89, 0x00])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RBX }, GPR::RAX, &[0x48, 0x89, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RBX }, GPR::RAX, &[0x48, 0x89, 0xC3])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R10 }, GPR::RBX, &[0x49, 0x89, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::RDX }, GPR::R11, &[0x4C, 0x89, 0xDA])]
#[case::gpr(GPROrMemory::GPR { gpr: GPR::R12 }, GPR::R11, &[0x4D, 0x89, 0xDC])]
fn test_mov_rm64_reg64(#[case] rm64: GPROrMemory, #[case] reg64: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { mov::encode_mov_rm64_reg64(rm64, reg64) };
    assert_encoded_instruction_eq(expected, &instr);
}
