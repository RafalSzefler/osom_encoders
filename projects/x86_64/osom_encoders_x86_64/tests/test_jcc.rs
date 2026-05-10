use rstest::rstest;

use osom_encoders_x86_64::constants::*;
use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

// Test cases for JCC instructions (short jumps)
#[rstest]
#[case(-1, &[0x77, 0xFF])] // JA
#[case(0, &[0x77, 0x00])]
#[case(1, &[0x77, 0x01])]
fn test_jcc_short_a(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_a_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x73, 0xFF])] // JAE
#[case(0, &[0x73, 0x00])]
#[case(1, &[0x73, 0x01])]
fn test_jcc_short_ae(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_ae_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x72, 0xFF])] // JB
#[case(0, &[0x72, 0x00])]
#[case(1, &[0x72, 0x01])]
fn test_jcc_short_b(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_b_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x76, 0xFF])] // JBE
#[case(0, &[0x76, 0x00])]
#[case(1, &[0x76, 0x01])]
fn test_jcc_short_be(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_be_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x74, 0xFF])] // JE
#[case(0, &[0x74, 0x00])]
#[case(1, &[0x74, 0x01])]
fn test_jcc_short_e(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_e_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x7F, 0xFF])] // JG
#[case(0, &[0x7F, 0x00])]
#[case(1, &[0x7F, 0x01])]
fn test_jcc_short_g(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_g_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(-1, &[0x7D, 0xFF])] // JGE
#[case(0, &[0x7D, 0x00])]
#[case(1, &[0x7D, 0x01])]
fn test_jcc_short_ge(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_ge_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x7C, 0xFF])] // JL
#[case(0, &[0x7C, 0x00])]
#[case(1, &[0x7C, 0x01])]
fn test_jcc_short_l(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_l_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x7E, 0xFF])] // JLE
#[case(0, &[0x7E, 0x00])]
#[case(1, &[0x7E, 0x01])]
fn test_jcc_short_le(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_le_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x75, 0xFF])] // JNE
#[case(0, &[0x75, 0x00])]
#[case(1, &[0x75, 0x01])]
fn test_jcc_short_ne(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_ne_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x71, 0xFF])] // JNO
#[case(0, &[0x71, 0x00])]
#[case(1, &[0x71, 0x01])]
fn test_jcc_short_no(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_no_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x7B, 0xFF])] // JNP
#[case(0, &[0x7B, 0x00])]
#[case(1, &[0x7B, 0x01])]
fn test_jcc_short_np(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_np_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x79, 0xFF])] // JNS
#[case(0, &[0x79, 0x00])]
#[case(1, &[0x79, 0x01])]
fn test_jcc_short_ns(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_ns_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x70, 0xFF])] // JO
#[case(0, &[0x70, 0x00])]
#[case(1, &[0x70, 0x01])]
fn test_jcc_short_o(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_o_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x7A, 0xFF])] // JP
#[case(0, &[0x7A, 0x00])]
#[case(1, &[0x7A, 0x01])]
fn test_jcc_short_p(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_p_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x78, 0xFF])] // JS
#[case(0, &[0x78, 0x00])]
#[case(1, &[0x78, 0x01])]
fn test_jcc_short_s(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jcc::encode_s_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

// Test cases for JCC instructions (long jumps)
#[rstest]
#[case(-1, &[0x0F, 0x87, 0xFF, 0xFF, 0xFF, 0xFF])] // JA
#[case(0, &[0x0F, 0x87, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x87, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_a(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_a_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x83, 0xFF, 0xFF, 0xFF, 0xFF])] // JAE
#[case(0, &[0x0F, 0x83, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x83, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ae(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_ae_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x82, 0xFF, 0xFF, 0xFF, 0xFF])] // JB
#[case(0, &[0x0F, 0x82, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x82, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_b(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_b_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x86, 0xFF, 0xFF, 0xFF, 0xFF])] // JBE
#[case(0, &[0x0F, 0x86, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x86, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_be(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_be_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x84, 0xFF, 0xFF, 0xFF, 0xFF])] // JE
#[case(0, &[0x0F, 0x84, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x84, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_e(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_e_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x8F, 0xFF, 0xFF, 0xFF, 0xFF])] // JG
#[case(0, &[0x0F, 0x8F, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8F, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_g(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_g_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x8D, 0xFF, 0xFF, 0xFF, 0xFF])] // JGE
#[case(0, &[0x0F, 0x8D, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8D, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ge(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_ge_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x8C, 0xFF, 0xFF, 0xFF, 0xFF])] // JL
#[case(0, &[0x0F, 0x8C, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8C, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_l(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_l_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x8E, 0xFF, 0xFF, 0xFF, 0xFF])] // JLE
#[case(0, &[0x0F, 0x8E, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8E, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_le(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_le_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x85, 0xFF, 0xFF, 0xFF, 0xFF])] // JNE
#[case(0, &[0x0F, 0x85, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x85, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ne(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_ne_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x81, 0xFF, 0xFF, 0xFF, 0xFF])] // JNO
#[case(0, &[0x0F, 0x81, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x81, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_no(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_no_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x8B, 0xFF, 0xFF, 0xFF, 0xFF])] // JNP
#[case(0, &[0x0F, 0x8B, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8B, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_np(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_np_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x89, 0xFF, 0xFF, 0xFF, 0xFF])] // JNS
#[case(0, &[0x0F, 0x89, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x89, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_ns(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_ns_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x80, 0xFF, 0xFF, 0xFF, 0xFF])] // JO
#[case(0, &[0x0F, 0x80, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x80, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_o(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_o_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x8A, 0xFF, 0xFF, 0xFF, 0xFF])] // JP
#[case(0, &[0x0F, 0x8A, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x8A, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_p(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_p_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x88, 0xFF, 0xFF, 0xFF, 0xFF])] // JS
#[case(0, &[0x0F, 0x88, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x88, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_s(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jcc::encode_s_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}

#[rstest]
#[case(-1, &[0x77, 0xFF])] // JA
#[case(0, &[0x77, 0x00])]
#[case(1, &[0x77, 0x01])]
fn test_jcc_short_a_patch(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let mut instr = unsafe { jcc::encode_a_imm8(0u8.into()) };
    instr.as_slice_mut()[JCC_SHORT_IMM8_OFFSET..].copy_from_slice(&imm8.encode());
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_SHORT_LENGTH);
}

#[rstest]
#[case(-1, &[0x0F, 0x88, 0xFF, 0xFF, 0xFF, 0xFF])] // JS
#[case(0, &[0x0F, 0x88, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0x0F, 0x88, 0x01, 0x00, 0x00, 0x00])]
fn test_jcc_long_s_patch(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let mut instr = unsafe { jcc::encode_s_imm32(0.into()) };
    instr.as_slice_mut()[JCC_LONG_IMM32_OFFSET..].copy_from_slice(&imm32.encode());
    assert_eq!(instr.as_slice(), expected);
    assert_eq!(instr.as_slice().len(), JCC_LONG_LENGTH);
}
