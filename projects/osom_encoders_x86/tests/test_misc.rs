mod common;

use rstest::rstest;

use osom_encoders_x86::encoding::*;
use osom_encoders_x86::models::*;

#[test]
fn test_encode_ret() {
    let instr = universal::encode_ret();
    common::assert_encoded_instruction_eq(&[0xC3], &instr);
}

#[rstest]
#[case(0x00, &[0xC2, 0x00, 0x00])]
#[case(0x1234, &[0xC2, 0x34, 0x12])]
fn test_encode_ret_imm16(#[case] imm16: u16, #[case] expected: &[u8]) {
    let imm16 = Immediate16::from_u16(imm16);
    let instr = universal::encode_ret_imm16(imm16);
    common::assert_encoded_instruction_eq(expected, &instr);
}

#[test]
fn test_encode_nop() {
    let instr = universal::encode_nop();
    common::assert_encoded_instruction_eq(&[0x90], &instr);
}

#[rstest]
#[case(1, &[0x90])]
#[case(2, &[0x66, 0x90])]
#[case(3, &[0x0F, 0x1F, 0x00])]
#[case(4, &[0x0F, 0x1F, 0x40, 0x00])]
#[case(5, &[0x0F, 0x1F, 0x44, 0x00, 0x00])]
#[case(6, &[0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00])]
#[case(7, &[0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00])]
#[case(8, &[0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00])]
#[case(9, &[0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00])]
fn test_encode_nop_with_length(#[case] length: u8, #[case] expected: &[u8]) {
    let instr = unsafe { universal::encode_nop_with_length(length) };
    common::assert_encoded_instruction_eq(expected, &instr);
}

#[cfg(debug_assertions)]
#[rstest]
#[case(0)]
#[case(10)]
#[case(11)]
#[case(12)]
#[case(13)]
#[case(14)]
#[case(15)]
#[case(16)]
#[case(17)]
#[case(100)]
#[case(253)]
#[case(254)]
#[case(255)]
#[should_panic]
fn test_encode_nop_with_length_panic(#[case] length: u8) {
    let _ = unsafe { universal::encode_nop_with_length(length) };
}

#[test]
fn test_encode_lock() {
    let instr = universal::encode_lock();
    common::assert_encoded_instruction_eq(&[0xF0], &instr);
}
