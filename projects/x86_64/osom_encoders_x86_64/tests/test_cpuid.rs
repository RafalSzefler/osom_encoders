use osom_encoders_x86_64::encoders::*;

#[test]
fn test_encode_cpuid() {
    let instr = unsafe { cpuid::encode() };
    assert_eq!(instr.as_slice(), &[0x0F, 0xA2]);
}
