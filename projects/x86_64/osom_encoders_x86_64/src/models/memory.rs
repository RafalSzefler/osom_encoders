use super::{GPR, Offset, Scale};

/// Represents a memory operand.
///
/// # Safety
///
/// Not all combinations of are valid for a given instruction.
/// For specifics refer to the Intel x86 manual.
///
/// In particular all the GPRs have to be 64-bit wide.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u8)]
#[must_use]
pub enum Memory {
    /// Represents a based memory operand, i.e. `[base + offset]`.
    Based { base: GPR, offset: Offset } = 0,

    /// Represents a scaled memory operand, i.e. `[index * scale + offset]`.
    ///
    /// # Safety
    ///
    /// `index == GPR::RSP` is not allowed.
    Scaled { index: GPR, scale: Scale, offset: Offset } = 1,

    /// Represents a based scaled memory operand, i.e. `[base + index * scale + offset]`.
    ///
    /// # Safety
    ///
    /// `index == GPR::RSP` is not allowed.
    BasedAndScaled {
        base: GPR,
        index: GPR,
        scale: Scale,
        offset: Offset,
    } = 2,

    /// Represents a RIP-relative memory operand, i.e. `[RIP + offset]`.
    RelativeToRIP { offset: Offset } = 3,
}

#[must_use]
pub(crate) struct BaseIndexIsExtended {
    pub base_is_extended: bool,
    pub index_is_extended: bool,
}

impl Memory {
    pub(crate) const fn base_index_is_extended(self) -> BaseIndexIsExtended {
        match self {
            Self::Based { base, .. } => BaseIndexIsExtended {
                base_is_extended: base.is_extended(),
                index_is_extended: false,
            },
            Self::Scaled { index, .. } => BaseIndexIsExtended {
                base_is_extended: false,
                index_is_extended: index.is_extended(),
            },
            Self::BasedAndScaled { base, index, .. } => BaseIndexIsExtended {
                base_is_extended: base.is_extended(),
                index_is_extended: index.is_extended(),
            },
            Self::RelativeToRIP { .. } => BaseIndexIsExtended {
                base_is_extended: false,
                index_is_extended: false,
            },
        }
    }
}
