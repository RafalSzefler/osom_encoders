use super::{GPR, Memory};

/// Represents a GPR or memory operand.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
#[repr(u8)]
pub enum GPROrMemory {
    GPR { gpr: GPR } = 1,
    Memory { memory: Memory } = 2,
}

impl From<GPR> for GPROrMemory {
    fn from(gpr: GPR) -> Self {
        GPROrMemory::GPR { gpr }
    }
}

impl From<Memory> for GPROrMemory {
    fn from(memory: Memory) -> Self {
        GPROrMemory::Memory { memory }
    }
}
