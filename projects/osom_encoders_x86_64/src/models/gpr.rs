use core::num::NonZero;

use osom_encoders_common::osom_debug_assert;

use super::Size;

/// Represents the kind of a general purpose register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[must_use]
pub enum GPRKind {
    /// Represents AH, BH, CH and DH registers.
    Bit8High = 1, // We start from 1 to allow Option<GPRKind> optimization

    /// Represents AL, CL, DL, BL, SPL, BPL, SIL, DIL, R8B, R9B, R10B, R11B, R12B, R13B, R14B and R15B registers.
    Bit8,

    /// Represents AX, CX, DX, BX, SP, BP, SI, DI, R8W, R9W, R10W, R11W, R12W, R13W, R14W and R15W registers.
    Bit16,

    /// Represents EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, R8D, R9D, R10D, R11D, R12D, R13D, R14D and R15D registers.
    Bit32,

    /// Represents RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14 and R15 registers.
    Bit64,
}

impl GPRKind {
    #[inline(always)]
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        let result = self as u8;
        unsafe { core::hint::assert_unchecked(result > 0 && result < 8) };
        result
    }

    /// Creates a new `GPRKind` from a `u8` index.
    ///
    /// # Safety
    ///
    /// The index must be in the range `1..=5`, otherwise the behavior is undefined.
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        osom_debug_assert!(value > 0 && value < 8);
        unsafe { core::mem::transmute(value) }
    }

    /// Creates a new `GPRKind` from a `u8` index.
    ///
    /// # Returns
    ///
    /// - `Some(GPRKind)` if the `u8` is in the range `1..=5`
    /// - `None` if the `u8` is 0 or greater than 5
    #[must_use]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value == 0 || value >= 8 {
            return None;
        }

        Some(unsafe { Self::from_u8_unchecked(value) })
    }

    /// The same as `==` operator but const.
    #[inline(always)]
    #[must_use]
    pub const fn equals(self, other: Self) -> bool {
        self.as_u8() == other.as_u8()
    }

    #[inline(always)]
    pub const fn size(self) -> Size {
        match self {
            GPRKind::Bit8 | GPRKind::Bit8High => Size::Bit8,
            GPRKind::Bit16 => Size::Bit16,
            GPRKind::Bit32 => Size::Bit32,
            GPRKind::Bit64 => Size::Bit64,
        }
    }
}

impl core::hash::Hash for GPRKind {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_u8().hash(state);
    }
}

/// Represents general purpose registers.
///
/// Internally stored as a non-zero `u8` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct GPR {
    value: NonZero<u8>,
}

const GPR_KIND_SHIFT: u8 = 5;

/// Error when creating a new `GPR`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NewGPRError {
    /// Error when creating a new `GPR` from a [`GPRKind::Bit8High`] and `index` outside of the `4..=7` range.
    InvalidBit8HighIndex,

    /// Error when creating a new `GPR` from a `kind` and `index` outside of the `0..=31` range.
    IndexOutOfRange,
}

macro_rules! gpr {
    ($name: ident, $kind: ident, $index: literal) => {
        pub const $name: Self = unsafe { Self::new_unchecked(GPRKind::$kind, $index) };
    };
}

impl GPR {
    gpr!(RAX, Bit64, 0);
    gpr!(RCX, Bit64, 1);
    gpr!(RDX, Bit64, 2);
    gpr!(RBX, Bit64, 3);
    gpr!(RSP, Bit64, 4);
    gpr!(RBP, Bit64, 5);
    gpr!(RSI, Bit64, 6);
    gpr!(RDI, Bit64, 7);
    gpr!(R8, Bit64, 8);
    gpr!(R9, Bit64, 9);
    gpr!(R10, Bit64, 10);
    gpr!(R11, Bit64, 11);
    gpr!(R12, Bit64, 12);
    gpr!(R13, Bit64, 13);
    gpr!(R14, Bit64, 14);
    gpr!(R15, Bit64, 15);

    gpr!(EAX, Bit32, 0);
    gpr!(ECX, Bit32, 1);
    gpr!(EDX, Bit32, 2);
    gpr!(EBX, Bit32, 3);
    gpr!(ESP, Bit32, 4);
    gpr!(EBP, Bit32, 5);
    gpr!(ESI, Bit32, 6);
    gpr!(EDI, Bit32, 7);
    gpr!(R8D, Bit32, 8);
    gpr!(R9D, Bit32, 9);
    gpr!(R10D, Bit32, 10);
    gpr!(R11D, Bit32, 11);
    gpr!(R12D, Bit32, 12);
    gpr!(R13D, Bit32, 13);
    gpr!(R14D, Bit32, 14);
    gpr!(R15D, Bit32, 15);

    gpr!(AX, Bit16, 0);
    gpr!(CX, Bit16, 1);
    gpr!(DX, Bit16, 2);
    gpr!(BX, Bit16, 3);
    gpr!(SP, Bit16, 4);
    gpr!(BP, Bit16, 5);
    gpr!(SI, Bit16, 6);
    gpr!(DI, Bit16, 7);
    gpr!(R8W, Bit16, 8);
    gpr!(R9W, Bit16, 9);
    gpr!(R10W, Bit16, 10);
    gpr!(R11W, Bit16, 11);
    gpr!(R12W, Bit16, 12);
    gpr!(R13W, Bit16, 13);
    gpr!(R14W, Bit16, 14);
    gpr!(R15W, Bit16, 15);

    gpr!(AL, Bit8, 0);
    gpr!(CL, Bit8, 1);
    gpr!(DL, Bit8, 2);
    gpr!(BL, Bit8, 3);
    gpr!(SPL, Bit8, 4);
    gpr!(BPL, Bit8, 5);
    gpr!(SIL, Bit8, 6);
    gpr!(DIL, Bit8, 7);
    gpr!(R8B, Bit8, 8);
    gpr!(R9B, Bit8, 9);
    gpr!(R10B, Bit8, 10);
    gpr!(R11B, Bit8, 11);
    gpr!(R12B, Bit8, 12);
    gpr!(R13B, Bit8, 13);
    gpr!(R14B, Bit8, 14);
    gpr!(R15B, Bit8, 15);

    gpr!(AH, Bit8High, 4);
    gpr!(CH, Bit8High, 5);
    gpr!(DH, Bit8High, 6);
    gpr!(BH, Bit8High, 7);

    /// Creates a new `GPR` from a `GPRKind` and an `index`.
    ///
    /// # Safety
    ///
    /// If `kind` is `Bit8High`, then `index` must be in the range `4..=7`.
    /// These represent AH, CH, DH and BH registers. All other combinations
    /// are valid.
    #[inline(always)]
    pub const unsafe fn new_unchecked(kind: GPRKind, index: u8) -> Self {
        osom_debug_assert!(!kind.equals(GPRKind::Bit8High) || (index >= 4 && index <= 7));
        osom_debug_assert!(index < 32);

        let kind_u8 = kind.as_u8();

        let value = (kind_u8 << GPR_KIND_SHIFT) | index;

        unsafe {
            Self {
                value: NonZero::new_unchecked(value),
            }
        }
    }

    /// Creates a new `GPR` from a `GPRKind` and an `index`.
    ///
    /// # Returns
    ///
    /// - `Ok(GPR)` if the `GPRKind` and `index` are valid
    /// - `Err(NewGPRError)` if the `kind` is [`GPRKind::Bit8High`] and
    ///   `index` is outside the `4..=7` range
    #[inline(always)]
    pub const fn new(kind: GPRKind, index: u8) -> Result<Self, NewGPRError> {
        if kind.equals(GPRKind::Bit8High) && (index < 4 || index > 7) {
            return Err(NewGPRError::InvalidBit8HighIndex);
        }

        if index >= 32 {
            return Err(NewGPRError::IndexOutOfRange);
        }

        Ok(unsafe { Self::new_unchecked(kind, index) })
    }

    #[inline(always)]
    pub const fn kind(self) -> GPRKind {
        let kind_u8 = self.value.get() >> GPR_KIND_SHIFT;
        unsafe { GPRKind::from_u8_unchecked(kind_u8) }
    }

    #[inline(always)]
    pub const fn size(self) -> Size {
        self.kind().size()
    }

    #[inline(always)]
    #[must_use]
    pub(crate) const fn index(self) -> u8 {
        self.value.get() & 0b11111
    }

    /// Returns true if the GPR is an extended X64 register.
    #[inline(always)]
    #[must_use]
    pub(crate) const fn is_extended(self) -> bool {
        self.index() > 0b111
    }

    #[allow(dead_code)] // TODO: will be used once we support APX.
    /// Returns true if the GPR is an extended register
    /// from Advanced Performance Extensions. Implies [`Self::is_extended`].
    #[inline(always)]
    #[must_use]
    pub(crate) const fn is_apx(self) -> bool {
        self.index() > 0b1111
    }

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
    pub const fn equals(&self, other: &Self) -> bool {
        self.value.get() == other.value.get()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(GPR::RAX, Size::Bit64)]
    #[case(GPR::EAX, Size::Bit32)]
    #[case(GPR::AX, Size::Bit16)]
    #[case(GPR::AL, Size::Bit8)]
    #[case(GPR::AH, Size::Bit8)]
    #[case(GPR::R8, Size::Bit64)]
    #[case(GPR::R8D, Size::Bit32)]
    #[case(GPR::R8W, Size::Bit16)]
    #[case(GPR::R8B, Size::Bit8)]
    fn test_gpr_size(#[case] gpr: GPR, #[case] size: Size) {
        assert_eq!(gpr.size(), size);
    }
}
