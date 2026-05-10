use osom_encoders_x86_64::encoders::nop::NopLength;
use rstest::rstest;

use osom_encoders_x86_64::encoders::*;

#[test]
fn test_encode_nop() {
    let instr = unsafe { nop::encode() };
    assert_eq!(instr.as_slice(), &[0x90]);
}

#[rstest]
#[case(0, &[])]
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
    let len = NopLength::new(length).unwrap();
    let instr = unsafe { nop::encode_with_len(len) };
    assert_eq!(instr.as_slice(), expected);
}

#[rstest]
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
fn test_encode_nop_with_length_panics(#[case] length: u8) {
    let len = unsafe { NopLength::new_unchecked(length) };
    let _ = unsafe { nop::encode_with_len(len) };
}
