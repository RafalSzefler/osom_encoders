#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]
mod common;

use common::asserts::assert_encoded_instruction_eq;

use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(0, &[0xCD, 0x00])]
#[case(1, &[0xCD, 0x01])]
#[case(2, &[0xCD, 0x02])]
#[case(3, &[0xCD, 0x03])]
#[case(10, &[0xCD, 0x0A])]
#[case(127, &[0xCD, 0x7F])]
#[case(128, &[0xCD, 0x80])]
#[case(255, &[0xCD, 0xFF])]
fn test_encode_int_imm8(#[case] imm8: u8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_u8(imm8);
    let instr = singleton::encode_int_imm8(imm8);
    assert_encoded_instruction_eq(expected, &instr);
}

#[test]
fn test_encode_int1() {
    let instr = singleton::encode_int1();
    assert_encoded_instruction_eq(&[0xF1], &instr);
}

#[test]
fn test_encode_int3() {
    let instr = singleton::encode_int3();
    assert_encoded_instruction_eq(&[0xCC], &instr);
}
