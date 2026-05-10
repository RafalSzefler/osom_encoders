use rstest::rstest;

use osom_encoders_x86_64::encoders::*;
use osom_encoders_x86_64::models::*;

#[rstest]
#[case(GPROrMemory::GPR { gpr: GPR::RAX }, &[0x8F, 0xC0])]
#[case::memory(GPROrMemory::Memory { memory: Memory::BasedAndScaled { base: GPR::RAX, index: GPR::RBX, scale: Scale::Scale4, offset: Offset::from_i8(1) } }, &[0x8F, 0x44, 0x98, 0x01])]
fn test_pop_rm64(#[case] rm64: GPROrMemory, #[case] expected: &[u8]) {
    let instr = unsafe { pop::encode_rm64(rm64) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
#[case(GPR::RAX, &[0x58])]
#[case(GPR::R10, &[0x41, 0x5A])]
fn test_pop_reg64(#[case] reg64: GPR, #[case] expected: &[u8]) {
    let instr = unsafe { pop::encode_reg64(reg64) };
    assert_eq!(instr.as_slice(), expected);
}
