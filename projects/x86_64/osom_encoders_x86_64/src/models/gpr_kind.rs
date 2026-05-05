use core::mem::transmute;

use super::Size;

/// Represents the kind of a general purpose register.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[must_use]
pub enum GPRKind {
    /// Represents AH, BH, CH and DH registers.
    Bit8High = 1, // We start from 1 to allow Option<GPRKind> optimization

    /// Represents AL, CL, DL, BL, SPL, BPL, SIL, DIL, R8B, R9B, R10B, R11B, R12B, R13B, R14B and R15B registers.
    Bit8 = 2,

    /// Represents AX, CX, DX, BX, SP, BP, SI, DI, R8W, R9W, R10W, R11W, R12W, R13W, R14W and R15W registers.
    Bit16 = 3,

    /// Represents EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, R8D, R9D, R10D, R11D, R12D, R13D, R14D and R15D registers.
    Bit32 = 4,

    /// Represents RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14 and R15 registers.
    Bit64 = 5,
}

impl GPRKind {
    /// Compares two [`GPRKind`] values for equality.
    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }

    /// Represents the size of the [`GPRKind`].
    pub const fn size(self) -> Size {
        match self {
            Self::Bit8High | Self::Bit8 => Size::Bit8,
            Self::Bit16 => Size::Bit16,
            Self::Bit32 => Size::Bit32,
            Self::Bit64 => Size::Bit64,
        }
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn as_u8(self) -> u8 {
        unsafe {
            let result = transmute::<Self, u8>(self);
            core::hint::assert_unchecked(result > 0);
            core::hint::assert_unchecked(result <= 5);
            result
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) const fn from_u8(value: u8) -> Self {
        debug_assert!(value > 0 && value <= 5, "Invalid GPRKind value");
        unsafe { core::mem::transmute(value) }
    }
}
