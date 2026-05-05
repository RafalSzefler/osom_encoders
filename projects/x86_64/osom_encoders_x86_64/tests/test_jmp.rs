use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(-1, &[0xEB, 0xFF])]
#[case(0, &[0xEB, 0x00])]
#[case(1, &[0xEB, 0x01])]
fn test_jmp_short(#[case] imm8: i8, #[case] expected: &[u8]) {
    let imm8 = Immediate8::from_i8(imm8);
    let instr = unsafe { jmp::encode_imm8(imm8) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(-1, &[0xE9, 0xFF, 0xFF, 0xFF, 0xFF])]
#[case(0, &[0xE9, 0x00, 0x00, 0x00, 0x00])]
#[case(1, &[0xE9, 0x01, 0x00, 0x00, 0x00])]
fn test_jmp_long(#[case] imm32: i32, #[case] expected: &[u8]) {
    let imm32 = Immediate32::from_i32(imm32);
    let instr = unsafe { jmp::encode_imm32(imm32) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0xFF, 0xE0])]
#[case(GPROrMemory::Memory { memory: Memory::Based { base: GPR::RBX, offset: Offset::None } }, &[0xFF, 0x23])]
#[case(GPROrMemory::Memory { memory: Memory::Based { base: GPR::R15, offset: Offset::from_i8(7) } }, &[0x41, 0xFF, 0x67, 0x07])]
fn test_jmp_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { jmp::encode_rm64(rm64) };
    assert_eq!(instr.as_slice(), expected);
}
