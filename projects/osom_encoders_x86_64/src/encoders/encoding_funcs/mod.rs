//! Holds encoding functions for a subset of X86 instructions.

mod mnemonics_gen;

/// Holds mnemonics for X86 instructions utilized by the crate.
pub mod mnemonics {
    pub use super::mnemonics_gen::*;
}

// Reexport.
mod enc;
pub use enc::*;

mod enc_gen;
pub use enc_gen::*;
