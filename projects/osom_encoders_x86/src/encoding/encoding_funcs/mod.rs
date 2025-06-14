//! Holds encoding functions for a subset of X86 instructions.

mod mnemonics_gen;

pub mod mnemonics {
    pub use super::mnemonics_gen::*;
}

// The following three modules will be reexported in the
// corresponding non-ext modules.
mod bit32_gen;
mod bit64_gen;
mod universal_gen;

pub mod bit32;
pub mod bit64;
pub mod universal;
