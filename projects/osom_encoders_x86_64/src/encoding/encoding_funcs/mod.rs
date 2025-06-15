//! Holds encoding functions for a subset of X86 instructions.

mod mnemonics_gen;

pub mod mnemonics {
    pub use super::mnemonics_gen::*;
}

// The following three modules will be reexported in the
// corresponding non-ext modules.
mod enc_gen;

pub mod enc;
