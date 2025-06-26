use core::mem::size_of;

use crate::models::{
    GPR, GPRKind, GPROrMemory, Immediate8, Immediate16, Immediate32, Immediate64, Memory, Offset, Scale, Size,
};

const _INVARIANTS: () = {
    assert!(size_of::<Size>() == 1, "Expected Size to be 1 byte");
    assert!(size_of::<Option<Size>>() == 1, "Expected Option<Size> to be 1 byte");
    assert!(size_of::<GPR>() == 1, "Expected GPR to be 1 byte");
    assert!(size_of::<Option<GPR>>() == 1, "Expected Option<GPR> to be 1 byte");
    assert!(size_of::<GPRKind>() == 1, "Expected GPRKind to be 1 byte");
    assert!(
        size_of::<Option<GPRKind>>() == 1,
        "Expected Option<GPRKind> to be 1 byte"
    );

    assert!(size_of::<Scale>() == 1, "Expected Scale to be 1 byte");
    assert!(size_of::<Option<Scale>>() == 1, "Expected Option<Scale> to be 1 byte");
    assert!(size_of::<Offset>() == 8, "Expected Displacement to be 8 bytes");
    assert!(
        size_of::<Option<Offset>>() == 8,
        "Expected Option<Displacement> to be 8 bytes"
    );
    assert!(size_of::<Memory>() <= 16, "Expected Memory to be at most 16 bytes");
    assert!(
        size_of::<GPROrMemory>() <= 16,
        "Expected GPROrMemory to be at most 16 bytes"
    );

    assert!(size_of::<Immediate8>() == 1, "Expected Immediate8 to be 1 byte");
    assert!(size_of::<Immediate16>() == 2, "Expected Immediate16 to be 2 bytes");
    assert!(size_of::<Immediate32>() == 4, "Expected Immediate32 to be 4 bytes");
    assert!(size_of::<Immediate64>() == 8, "Expected Immediate64 to be 8 bytes");
};
