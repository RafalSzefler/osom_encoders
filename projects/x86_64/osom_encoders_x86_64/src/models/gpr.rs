use core::mem::transmute;

use super::{GPRKind, Size};

/// Represents a general purpose register in the `X86_64` instruction set.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[must_use]
pub enum GPR {
    RAX = 1, // We start from 1 to allow Option<GPR> optimization
    RBX = 2,
    RCX = 3,
    RDX = 4,
    RSI = 5,
    RDI = 6,
    RBP = 7,
    RSP = 8,
    R8 = 9,
    R9 = 10,
    R10 = 11,
    R11 = 12,
    R12 = 13,
    R13 = 14,
    R14 = 15,
    R15 = 16,

    EAX = 17,
    EBX = 18,
    ECX = 19,
    EDX = 20,
    ESI = 21,
    EDI = 22,
    EBP = 23,
    ESP = 24,
    R8D = 25,
    R9D = 26,
    R10D = 27,
    R11D = 28,
    R12D = 29,
    R13D = 30,
    R14D = 31,
    R15D = 32,

    AX = 33,
    BX = 34,
    CX = 35,
    DX = 36,
    SI = 37,
    DI = 38,
    BP = 39,
    SP = 40,
    R8W = 41,
    R9W = 42,
    R10W = 43,
    R11W = 44,
    R12W = 45,
    R13W = 46,
    R14W = 47,
    R15W = 48,

    AL = 49,
    BL = 50,
    CL = 51,
    DL = 52,
    SIL = 53,
    DIL = 54,
    BPL = 55,
    SPL = 56,
    R8B = 57,
    R9B = 58,
    R10B = 59,
    R11B = 60,
    R12B = 61,
    R13B = 62,
    R14B = 63,
    R15B = 64,

    AH = 65,
    BH = 66,
    CH = 67,
    DH = 68,
}

impl GPR {
    /// Compares two [`GPR`] values for equality.
    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }

    /// Represents the kind of the [`GPR`].
    pub const fn kind(self) -> GPRKind {
        match self {
            Self::AH | Self::BH | Self::CH | Self::DH => GPRKind::Bit8High,
            Self::AL
            | Self::CL
            | Self::DL
            | Self::BL
            | Self::SPL
            | Self::BPL
            | Self::SIL
            | Self::DIL
            | Self::R8B
            | Self::R9B
            | Self::R10B
            | Self::R11B
            | Self::R12B
            | Self::R13B
            | Self::R14B
            | Self::R15B => GPRKind::Bit8,
            Self::AX
            | Self::CX
            | Self::DX
            | Self::BX
            | Self::SP
            | Self::BP
            | Self::SI
            | Self::DI
            | Self::R8W
            | Self::R9W
            | Self::R10W
            | Self::R11W
            | Self::R12W
            | Self::R13W
            | Self::R14W
            | Self::R15W => GPRKind::Bit16,
            Self::EAX
            | Self::ECX
            | Self::EDX
            | Self::EBX
            | Self::ESP
            | Self::EBP
            | Self::ESI
            | Self::EDI
            | Self::R8D
            | Self::R9D
            | Self::R10D
            | Self::R11D
            | Self::R12D
            | Self::R13D
            | Self::R14D
            | Self::R15D => GPRKind::Bit32,
            Self::RAX
            | Self::RCX
            | Self::RDX
            | Self::RBX
            | Self::RSP
            | Self::RBP
            | Self::RSI
            | Self::RDI
            | Self::R8
            | Self::R9
            | Self::R10
            | Self::R11
            | Self::R12
            | Self::R13
            | Self::R14
            | Self::R15 => GPRKind::Bit64,
        }
    }

    /// Represents the size of the [`GPR`].
    pub const fn size(self) -> Size {
        match self {
            Self::AH
            | Self::BH
            | Self::CH
            | Self::DH
            | Self::AL
            | Self::CL
            | Self::DL
            | Self::BL
            | Self::SPL
            | Self::BPL
            | Self::SIL
            | Self::DIL
            | Self::R8B
            | Self::R9B
            | Self::R10B
            | Self::R11B
            | Self::R12B
            | Self::R13B
            | Self::R14B
            | Self::R15B => Size::Bit8,
            Self::AX
            | Self::CX
            | Self::DX
            | Self::BX
            | Self::SP
            | Self::BP
            | Self::SI
            | Self::DI
            | Self::R8W
            | Self::R9W
            | Self::R10W
            | Self::R11W
            | Self::R12W
            | Self::R13W
            | Self::R14W
            | Self::R15W => Size::Bit16,
            Self::EAX
            | Self::ECX
            | Self::EDX
            | Self::EBX
            | Self::ESP
            | Self::EBP
            | Self::ESI
            | Self::EDI
            | Self::R8D
            | Self::R9D
            | Self::R10D
            | Self::R11D
            | Self::R12D
            | Self::R13D
            | Self::R14D
            | Self::R15D => Size::Bit32,
            Self::RAX
            | Self::RCX
            | Self::RDX
            | Self::RBX
            | Self::RSP
            | Self::RBP
            | Self::RSI
            | Self::RDI
            | Self::R8
            | Self::R9
            | Self::R10
            | Self::R11
            | Self::R12
            | Self::R13
            | Self::R14
            | Self::R15 => Size::Bit64,
        }
    }

    /// Returns the index of the [`GPR`]. The purpose of the index is to be used
    /// as a part of binary encoding, as used by Intel.
    #[must_use]
    pub(crate) const fn index(self) -> u8 {
        match self {
            Self::RAX | Self::EAX | Self::AX | Self::AL => 0,
            Self::RCX | Self::ECX | Self::CX | Self::CL => 1,
            Self::RDX | Self::EDX | Self::DX | Self::DL => 2,
            Self::RBX | Self::EBX | Self::BX | Self::BL => 3,
            Self::RSP | Self::ESP | Self::SP | Self::SPL | Self::AH => 4,
            Self::RBP | Self::EBP | Self::BP | Self::BPL | Self::CH => 5,
            Self::RSI | Self::ESI | Self::SI | Self::SIL | Self::DH => 6,
            Self::RDI | Self::EDI | Self::DI | Self::DIL | Self::BH => 7,
            Self::R8 | Self::R8D | Self::R8W | Self::R8B => 8,
            Self::R9 | Self::R9D | Self::R9W | Self::R9B => 9,
            Self::R10 | Self::R10D | Self::R10W | Self::R10B => 10,
            Self::R11 | Self::R11D | Self::R11W | Self::R11B => 11,
            Self::R12 | Self::R12D | Self::R12W | Self::R12B => 12,
            Self::R13 | Self::R13D | Self::R13W | Self::R13B => 13,
            Self::R14 | Self::R14D | Self::R14W | Self::R14B => 14,
            Self::R15 | Self::R15D | Self::R15W | Self::R15B => 15,
        }
    }

    /// Returns true if the [`GPR`] is extended. This typically means
    /// registers with numerical names, e.g. R8, R9, etc.
    #[inline(always)]
    #[must_use]
    pub(crate) const fn is_extended(self) -> bool {
        self.index() > 0b111
    }

    /// Lower 3 bits of the index of the [`GPR`].
    #[inline(always)]
    #[must_use]
    pub(crate) const fn lower_3_bits_index(self) -> u8 {
        self.index() & 0b111
    }

    /// This function verifies that the index of given GPR matches
    /// the index of AH, CH, DH or BH registers. Meaning it is in
    /// the `4..=7` range. This is important, since those registers
    /// share the index with SPL, BPL, SIL and DIL registers (which
    /// are of the same size). And have to be encoded differently,
    /// typically by using Operand Size Override prefix.
    #[inline(always)]
    #[must_use]
    pub(crate) const fn index_matches_bit8_high(self) -> bool {
        let idx = self.index();
        idx >= 4 && idx <= 7
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn as_u8(self) -> u8 {
        unsafe {
            let result = transmute::<Self, u8>(self);
            core::hint::assert_unchecked(result > 0);
            core::hint::assert_unchecked(result <= 68);
            result
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn from_u8(value: u8) -> Self {
        debug_assert!(value > 0 && value <= 68, "Invalid GPR value");
        unsafe { transmute(value) }
    }
}
