use osom_encoders_common::{U4, u4_values};
use rstest::rstest;

#[test]
fn test_u4_construction() {
    // Test valid values
    for i in 0..=15 {
        let new_u4 = U4::new(i).unwrap();
        let unsafe_u4 = unsafe { U4::new_unchecked(i) };
        assert_eq!(new_u4, unsafe_u4);
        assert_eq!(new_u4.as_u8(), i);
        assert_eq!(unsafe_u4.as_u8(), i);
    }
}

#[rstest]
#[case(16)]
#[case(17)]
#[case(18)]
#[case(19)]
#[case(20)]
#[case(21)]
#[case(22)]
#[case(23)]
#[case(24)]
#[case(253)]
#[case(254)]
#[case(255)]
fn test_u4_construction_invalid_values(#[case] value: u8) {
    assert!(U4::new(value).is_err());
}

#[rstest]
#[case(u4_values::ZERO, 0)]
#[case(u4_values::ONE, 1)]
#[case(u4_values::TWO, 2)]
#[case(u4_values::THREE, 3)]
#[case(u4_values::FOUR, 4)]
#[case(u4_values::FIVE, 5)]
#[case(u4_values::SIX, 6)]
#[case(u4_values::SEVEN, 7)]
#[case(u4_values::EIGHT, 8)]
#[case(u4_values::NINE, 9)]
#[case(u4_values::TEN, 10)]
#[case(u4_values::ELEVEN, 11)]
#[case(u4_values::TWELVE, 12)]
#[case(u4_values::THIRTEEN, 13)]
#[case(u4_values::FOURTEEN, 14)]
#[case(u4_values::FIFTEEN, 15)]
fn test_u4_constants(#[case] u4: U4, #[case] expected: u8) {
    assert_eq!(u4.as_u8(), expected);
}

#[test]
fn test_u4_bitwise_operations() {
    let a = unsafe { U4::new_unchecked(0b1010) }; // 10
    let b = unsafe { U4::new_unchecked(0b1100) }; // 12

    // Test AND
    assert_eq!((a & b).as_u8(), 0b1000);
    assert_eq!(a.binary_and(b).as_u8(), 0b1000);

    // Test OR
    assert_eq!((a | b).as_u8(), 0b1110);
    assert_eq!(a.binary_or(b).as_u8(), 0b1110);

    // Test XOR
    assert_eq!((a ^ b).as_u8(), 0b0110);
    assert_eq!(a.binary_xor(b).as_u8(), 0b0110);

    // Test NOT
    assert_eq!((!a).as_u8(), 0b0101);
    assert_eq!(a.binary_not().as_u8(), 0b0101);

    // Test shifts
    assert_eq!((a << 1).as_u8(), 0b0100);
    assert_eq!(a.binary_shl(1).as_u8(), 0b0100);
    assert_eq!((a >> 1).as_u8(), 0b0101);
    assert_eq!(a.binary_shr(1).as_u8(), 0b0101);
}

#[test]
fn test_u4_ordering() {
    let a = unsafe { U4::new_unchecked(5) };
    let b = unsafe { U4::new_unchecked(10) };
    let c = unsafe { U4::new_unchecked(5) };

    assert!(a < b);
    assert!(b > a);
    assert!(a == c);
    assert!(a <= c);
    assert!(a >= c);
}

#[test]
fn test_u4_debug() {
    let u4 = unsafe { U4::new_unchecked(7) };
    assert_eq!(format!("{:?}", u4), "U4 { value: 7 }");
}
