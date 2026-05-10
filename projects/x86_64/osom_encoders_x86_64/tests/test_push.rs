use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0xFF, 0xF0])]
#[case::memory(GPROrMemory::Memory { memory: Memory::BasedAndScaled { base: GPR::RAX, index: GPR::RBX, scale: Scale::Scale4, offset: Offset::from_i8(1) } }, &[0xFF, 0x74, 0x98, 0x01])]
fn test_push_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { push::encode_rm64(rm64) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(GPR::RAX, &[0x50])]
#[case(GPR::R10, &[0x41, 0x52])]
fn test_push_reg64(#[case] reg64: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { push::encode_reg64(reg64) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(0x7F, &[0x6A, 0x7F])]
fn test_push_imm8(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { push::encode_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(0x1234, &[0x66, 0x68, 0x34, 0x12])]
fn test_push_imm16(#[case] imm16: i16, #[case] expected: &[u8]) {
    let imm16 = Immediate16::from_i16(imm16);
    let instr = unsafe { push::encode_imm16(imm16) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(0x12345678, &[0x68, 0x78, 0x56, 0x34, 0x12])]
fn test_push_imm32(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { push::encode_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
}
