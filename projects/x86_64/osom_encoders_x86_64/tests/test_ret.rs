use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[test]
fn test_encode_ret() {
    let instr = unsafe { ret::encode() };
    assert_encoded_instruction_eq(&[0xC3], &instr);
}

#[rstest]
#[case(0x00, &[0xC2, 0x00, 0x00])]
#[case(0x1234, &[0xC2, 0x34, 0x12])]
fn test_encode_ret_imm16(#[case] imm16: u16, #[case] expected: &[u8]) {
    let imm16 = Immediate16::from_u16(imm16);
    let instr = unsafe { ret::encode_imm16(imm16) };
    assert_encoded_instruction_eq(expected, &instr);
}
