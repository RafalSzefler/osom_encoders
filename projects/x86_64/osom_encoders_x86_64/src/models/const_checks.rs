#![allow(clippy::unreadable_literal)]

use super::{
    EncodedX86_64Instruction, GPR, GPRKind, GPROrMemory, Immediate8, Immediate16, Immediate32, Immediate64, Memory,
    Offset, Scale, Size,
};

const _VALIDATE_IMMEDIATES: () = const {
    // Ensure that zero- and sign- extensions work correctly. At compile time.

    macro_rules! const_assert_eq {
        ($a:expr, $b:expr, $msg:literal) => {
            assert!(($a).equals($b), $msg);
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

const _VALIDATE_SIZES: () = const {
    assert!(
        size_of::<EncodedX86_64Instruction>() == 16,
        "EncodedX86_64Instruction size is not 16"
    );
    assert!(size_of::<Immediate8>() == 1, "Immediate8 size is not 1");
    assert!(size_of::<Immediate16>() == 2, "Immediate16 size is not 2");
    assert!(size_of::<Immediate32>() == 4, "Immediate32 size is not 4");
    assert!(size_of::<Immediate64>() == 8, "Immediate64 size is not 8");
    assert!(size_of::<Offset>() <= 8, "Offset size is greater than 8");
    assert!(
        size_of::<Option<Offset>>() <= 8,
        "Option<Offset> size is greater than 8"
    );
    assert!(size_of::<Scale>() == 1, "Scale size is not 1");
    assert!(size_of::<Option<Scale>>() == 1, "Option<Scale> size is not 1");
    assert!(size_of::<Size>() == 1, "Size size is not 1");
    assert!(size_of::<Option<Size>>() == 1, "Option<Size> size is not 1");
    assert!(size_of::<GPRKind>() == 1, "GPRKind size is not 1");
    assert!(size_of::<Option<GPRKind>>() == 1, "Option<GPRKind> size is not 1");
    assert!(size_of::<GPR>() == 1, "GPR size is not 1");
    assert!(size_of::<Option<GPR>>() == 1, "Option<GPR> size is not 1");
    assert!(size_of::<Memory>() <= 16, "Memory size is greater than 16");
    assert!(
        size_of::<Option<Memory>>() <= 16,
        "Option<Memory> size is greater than 16"
    );

    assert!(size_of::<GPROrMemory>() <= 16, "GPROrMemory size is greater than 16");
    assert!(
        size_of::<Option<GPROrMemory>>() <= 16,
        "Option<GPROrMemory> size is greater than 16"
    );
};
