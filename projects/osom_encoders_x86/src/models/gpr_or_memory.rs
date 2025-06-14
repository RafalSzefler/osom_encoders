use super::{GPR, Memory};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GPROrMemory {
    GPR { gpr: GPR },
    Memory { memory: Memory }
}
