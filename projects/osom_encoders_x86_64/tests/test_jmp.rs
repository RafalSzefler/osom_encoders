#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(-1, &[0xEB, 0xFF])]
#[case(0, &[0xEB, 0x00])]
#[case(1, &[0xEB, 0x01])]
fn test_jmp_short(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jmp::encode_jmp_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0xE9, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case(0, &[0xE9, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0xE9, 0x01, 0x00, 0x00, 0x00])]
fn test_jmp_long(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jmp::encode_jmp_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0xFF, 0xE0])]
#[case(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBX, offset: Offset::None } }, &[0xFF, 0x23])]
#[case(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R15, offset: Offset::from_i8(7) } }, &[0x41, 0xFF, 0x67, 0x07])]
fn test_jmp_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { jmp::encode_jmp_rm64(rm64) };
    assert_encoded_instruction_eq(expected, &instr);
}

// Test cases for JCC instructions (short jumps)
#[rstest]
#[case(-1, &[0x77, 0xFF])] // JA
#[case(0, &[0x77, 0x00])]
#[case(1, &[0x77, 0x01])]
fn test_jcc_short_a(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_A_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x73, 0xFF])] // JAE
#[case(0, &[0x73, 0x00])]
#[case(1, &[0x73, 0x01])]
fn test_jcc_short_ae(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_AE_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x72, 0xFF])] // JB
#[case(0, &[0x72, 0x00])]
#[case(1, &[0x72, 0x01])]
fn test_jcc_short_b(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_B_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x76, 0xFF])] // JBE
#[case(0, &[0x76, 0x00])]
#[case(1, &[0x76, 0x01])]
fn test_jcc_short_be(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_BE_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x74, 0xFF])] // JE
#[case(0, &[0x74, 0x00])]
#[case(1, &[0x74, 0x01])]
fn test_jcc_short_e(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_E_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x7F, 0xFF])] // JG
#[case(0, &[0x7F, 0x00])]
#[case(1, &[0x7F, 0x01])]
fn test_jcc_short_g(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_G_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x7D, 0xFF])] // JGE
#[case(0, &[0x7D, 0x00])]
#[case(1, &[0x7D, 0x01])]
fn test_jcc_short_ge(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_GE_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x7C, 0xFF])] // JL
#[case(0, &[0x7C, 0x00])]
#[case(1, &[0x7C, 0x01])]
fn test_jcc_short_l(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_L_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x7E, 0xFF])] // JLE
#[case(0, &[0x7E, 0x00])]
#[case(1, &[0x7E, 0x01])]
fn test_jcc_short_le(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_LE_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x75, 0xFF])] // JNE
#[case(0, &[0x75, 0x00])]
#[case(1, &[0x75, 0x01])]
fn test_jcc_short_ne(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_NE_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x71, 0xFF])] // JNO
#[case(0, &[0x71, 0x00])]
#[case(1, &[0x71, 0x01])]
fn test_jcc_short_no(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_NO_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x7B, 0xFF])] // JNP
#[case(0, &[0x7B, 0x00])]
#[case(1, &[0x7B, 0x01])]
fn test_jcc_short_np(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_NP_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x79, 0xFF])] // JNS
#[case(0, &[0x79, 0x00])]
#[case(1, &[0x79, 0x01])]
fn test_jcc_short_ns(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_NS_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x70, 0xFF])] // JO
#[case(0, &[0x70, 0x00])]
#[case(1, &[0x70, 0x01])]
fn test_jcc_short_o(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_O_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x7A, 0xFF])] // JP
#[case(0, &[0x7A, 0x00])]
#[case(1, &[0x7A, 0x01])]
fn test_jcc_short_p(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_P_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x78, 0xFF])] // JS
#[case(0, &[0x78, 0x00])]
#[case(1, &[0x78, 0x01])]
fn test_jcc_short_s(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = jcc::encode_jcc_S_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

// Test cases for JCC instructions (long jumps)
#[rstest]
#[case(-1, &[0x0F, 0x87, 0xFF, 0xFF, 0xFF, 0xFF])] // JA
#[case(0, &[0x0F, 0x87, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x87, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_a(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_A_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x83, 0xFF, 0xFF, 0xFF, 0xFF])] // JAE
#[case(0, &[0x0F, 0x83, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x83, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ae(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_AE_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x82, 0xFF, 0xFF, 0xFF, 0xFF])] // JB
#[case(0, &[0x0F, 0x82, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x82, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_b(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_B_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x86, 0xFF, 0xFF, 0xFF, 0xFF])] // JBE
#[case(0, &[0x0F, 0x86, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x86, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_be(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_BE_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x84, 0xFF, 0xFF, 0xFF, 0xFF])] // JE
#[case(0, &[0x0F, 0x84, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x84, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_e(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_E_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x8F, 0xFF, 0xFF, 0xFF, 0xFF])] // JG
#[case(0, &[0x0F, 0x8F, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8F, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_g(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_G_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x8D, 0xFF, 0xFF, 0xFF, 0xFF])] // JGE
#[case(0, &[0x0F, 0x8D, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8D, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ge(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_GE_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x8C, 0xFF, 0xFF, 0xFF, 0xFF])] // JL
#[case(0, &[0x0F, 0x8C, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8C, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_l(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_L_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x8E, 0xFF, 0xFF, 0xFF, 0xFF])] // JLE
#[case(0, &[0x0F, 0x8E, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8E, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_le(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_LE_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x85, 0xFF, 0xFF, 0xFF, 0xFF])] // JNE
#[case(0, &[0x0F, 0x85, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x85, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ne(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_NE_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x81, 0xFF, 0xFF, 0xFF, 0xFF])] // JNO
#[case(0, &[0x0F, 0x81, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x81, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_no(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_NO_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x8B, 0xFF, 0xFF, 0xFF, 0xFF])] // JNP
#[case(0, &[0x0F, 0x8B, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8B, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_np(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_NP_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x89, 0xFF, 0xFF, 0xFF, 0xFF])] // JNS
#[case(0, &[0x0F, 0x89, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x89, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ns(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_NS_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x80, 0xFF, 0xFF, 0xFF, 0xFF])] // JO
#[case(0, &[0x0F, 0x80, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x80, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_o(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_O_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x8A, 0xFF, 0xFF, 0xFF, 0xFF])] // JP
#[case(0, &[0x0F, 0x8A, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8A, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_p(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_P_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}

#[rstest]
#[case(-1, &[0x0F, 0x88, 0xFF, 0xFF, 0xFF, 0xFF])] // JS
#[case(0, &[0x0F, 0x88, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x88, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_s(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = jcc::encode_jcc_S_imm32(imm32);
    assert_encoded_instruction_eq(expected, &instr);
}
