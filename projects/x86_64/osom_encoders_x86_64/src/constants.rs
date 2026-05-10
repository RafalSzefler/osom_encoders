//! This module holds several useful constants.

/// Length of the short jump instruction in bytes.
pub const JMP_SHORT_LENGTH: usize = 2;

/// Offset of the immediate byte in the short jump instruction.
/// Useful for patching `jmp` instructions.
pub const JMP_SHORT_IMM8_OFFSET: usize = 1;

/// Length of the long jump instruction in bytes.
pub const JMP_LONG_LENGTH: usize = 5;

/// Offset of the immediate dword in the long jump instruction.
/// Useful for patching `jmp` instructions.
pub const JMP_LONG_IMM32_OFFSET: usize = 1;

/// Length of the short conditional jump instruction in bytes.
pub const JCC_SHORT_LENGTH: usize = 2;

/// Offset of the immediate byte in the short conditional jump instruction.
/// Useful for patching `jcc` instructions.
pub const JCC_SHORT_IMM8_OFFSET: usize = 1;

/// Length of the long conditional jump instruction in bytes.
pub const JCC_LONG_LENGTH: usize = 6;

/// Offset of the immediate dword in the long conditional jump instruction.
/// Useful for patching `jcc` instructions.
pub const JCC_LONG_IMM32_OFFSET: usize = 1;
