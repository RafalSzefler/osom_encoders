use super::{Immediate8, Immediate16, Immediate32, Immediate64};

const _VALIDATE: () = const {
    // Ensure that zero- and sign- extensions work correctly. At compile time.

    macro_rules! const_assert_eq {
        ($a:expr, $b:expr, $msg:literal) => {
            assert!($a.value() == $b.value(), $msg);
        };
    }

    const_assert_eq!(
        Immediate16::from_imm8_zero_extended(Immediate8::from_i8(0)),
        Immediate16::from_i16(0),
        "[1] Invalid Immediate16 zero extension"
    );
    const_assert_eq!(
        Immediate16::from_imm8_zero_extended(Immediate8::from_i8(1)),
        Immediate16::from_i16(1),
        "[2] Invalid Immediate16 zero extension"
    );
    const_assert_eq!(
        Immediate16::from_imm8_zero_extended(Immediate8::from_i8(-1)),
        Immediate16::from_i16(255),
        "[3] Invalid Immediate16 zero extension"
    );
    const_assert_eq!(
        Immediate16::from_imm8_sign_extended(Immediate8::from_i8(0)),
        Immediate16::from_i16(0),
        "[4] Invalid Immediate16 sign extension"
    );
    const_assert_eq!(
        Immediate16::from_imm8_sign_extended(Immediate8::from_i8(1)),
        Immediate16::from_i16(1),
        "[5] Invalid Immediate16 sign extension"
    );
    const_assert_eq!(
        Immediate16::from_imm8_sign_extended(Immediate8::from_i8(-1)),
        Immediate16::from_i16(-1),
        "[6] Invalid Immediate16 sign extension"
    );
    const_assert_eq!(
        Immediate32::from_imm8_zero_extended(Immediate8::from_i8(0)),
        Immediate32::from_i32(0),
        "[7] Invalid Immediate32 zero extension"
    );
    const_assert_eq!(
        Immediate32::from_imm8_zero_extended(Immediate8::from_i8(1)),
        Immediate32::from_i32(1),
        "[8] Invalid Immediate32 zero extension"
    );
    const_assert_eq!(
        Immediate32::from_imm8_zero_extended(Immediate8::from_i8(-1)),
        Immediate32::from_i32(255),
        "[9] Invalid Immediate32 zero extension"
    );
    const_assert_eq!(
        Immediate32::from_imm8_sign_extended(Immediate8::from_i8(0)),
        Immediate32::from_i32(0),
        "[10] Invalid Immediate32 sign extension"
    );
    const_assert_eq!(
        Immediate32::from_imm8_sign_extended(Immediate8::from_i8(1)),
        Immediate32::from_i32(1),
        "[11] Invalid Immediate32 sign extension"
    );
    const_assert_eq!(
        Immediate32::from_imm8_sign_extended(Immediate8::from_i8(-1)),
        Immediate32::from_i32(-1),
        "[12] Invalid Immediate32 sign extension"
    );
    const_assert_eq!(
        Immediate32::from_imm16_zero_extended(Immediate16::from_i16(0)),
        Immediate32::from_i32(0),
        "[13] Invalid Immediate32 zero extension"
    );
    const_assert_eq!(
        Immediate32::from_imm16_zero_extended(Immediate16::from_i16(1)),
        Immediate32::from_i32(1),
        "[14] Invalid Immediate32 zero extension"
    );
    const_assert_eq!(
        Immediate32::from_imm16_zero_extended(Immediate16::from_i16(-200)),
        Immediate32::from_i32(65336),
        "[15] Invalid Immediate32 zero extension"
    );
    const_assert_eq!(
        Immediate32::from_imm16_sign_extended(Immediate16::from_i16(1)),
        Immediate32::from_i32(1),
        "[16] Invalid Immediate32 sign extension"
    );
    const_assert_eq!(
        Immediate32::from_imm16_sign_extended(Immediate16::from_i16(-1)),
        Immediate32::from_i32(-1),
        "[17] Invalid Immediate32 sign extension"
    );
    const_assert_eq!(
        Immediate32::from_imm16_sign_extended(Immediate16::from_i16(-200)),
        Immediate32::from_i32(-200),
        "[18] Invalid Immediate32 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm8_zero_extended(Immediate8::from_i8(0)),
        Immediate64::from_i64(0),
        "[19] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm8_zero_extended(Immediate8::from_i8(1)),
        Immediate64::from_i64(1),
        "[20] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm8_zero_extended(Immediate8::from_i8(-1)),
        Immediate64::from_i64(255),
        "[21] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm8_sign_extended(Immediate8::from_i8(0)),
        Immediate64::from_i64(0),
        "[22] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm8_sign_extended(Immediate8::from_i8(1)),
        Immediate64::from_i64(1),
        "[23] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm8_sign_extended(Immediate8::from_i8(-1)),
        Immediate64::from_i64(-1),
        "[24] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm16_zero_extended(Immediate16::from_i16(0)),
        Immediate64::from_i64(0),
        "[25] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm16_zero_extended(Immediate16::from_i16(1)),
        Immediate64::from_i64(1),
        "[26] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm16_sign_extended(Immediate16::from_i16(1)),
        Immediate64::from_i64(1),
        "[27] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm16_sign_extended(Immediate16::from_i16(-1)),
        Immediate64::from_i64(-1),
        "[28] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm32_zero_extended(Immediate32::from_i32(0)),
        Immediate64::from_i64(0),
        "[29] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm32_zero_extended(Immediate32::from_i32(1)),
        Immediate64::from_i64(1),
        "[30] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm32_sign_extended(Immediate32::from_i32(1)),
        Immediate64::from_i64(1),
        "[31] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm32_sign_extended(Immediate32::from_i32(-1)),
        Immediate64::from_i64(-1),
        "[32] Invalid Immediate64 sign extension"
    );
    const_assert_eq!(
        Immediate64::from_imm32_zero_extended(Immediate32::from_i32(-800000)),
        Immediate64::from_i64(0xFFF3CB00),
        "[33] Invalid Immediate64 zero extension"
    );
    const_assert_eq!(
        Immediate64::from_imm32_sign_extended(Immediate32::from_i32(-800000)),
        Immediate64::from_i64(-800000),
        "[34] Invalid Immediate64 sign extension"
    );
};
