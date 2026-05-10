use std::collections::HashSet;

use serde::Deserialize;
use strum::{EnumIter, IntoStaticStr};

use super::validation::validate;

/// Represents various operand encodings. Values inside the enum
/// are just identifiers that require additional special interpretation.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, EnumIter, IntoStaticStr)]
#[must_use]
pub enum OperandEncodingId {
    /// Zero operand.
    ZO,

    /// Immediate operand.
    I,

    /// Immediate to memory/register operand.
    MI,

    /// Immediate to register operand.
    OI,

    /// Register operand to memory/register operand (or vice versa).
    MR,

    /// Single register or memory operand.
    M,

    /// Register encoded in opcode.
    O,
}

/// Represents various operands used by the `X86_64` instruction set.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, EnumIter, IntoStaticStr)]
#[must_use]
pub enum OperandId {
    /// 8-bit immediate value.
    Imm8,

    /// 16-bit immediate value.
    Imm16,

    /// 32-bit immediate value.
    Imm32,

    /// 64-bit immediate value.
    Imm64,

    /// 8-bit general purpose register or memory operand.
    RM8,

    /// 16-bit general purpose register or memory operand.
    RM16,

    /// 32-bit general purpose register or memory operand.
    RM32,

    /// 64-bit general purpose register or memory operand.
    RM64,

    /// 8-bit general purpose register.
    Reg8,

    /// 16-bit general purpose register.
    Reg16,

    /// 32-bit general purpose register.
    Reg32,

    /// 64-bit general purpose register.
    Reg64,

    /// 64-bit memory.
    Mem64,
}

/// Represents various flags used by the `X86_64` instruction set.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone, EnumIter, IntoStaticStr)]
#[must_use]
pub enum FlagId {
    /// Requires operand size override prefix.
    OSO,

    /// Requires REX.W prefix.
    RexW,
}

/// Full description of a flag.
#[derive(Debug, Deserialize)]
#[must_use]
pub struct FlagWithDescription {
    pub id: FlagId,
    pub description: String,
}

/// Full description of an operand encoding.
#[derive(Debug, Deserialize)]
#[must_use]
pub struct OperandEncoding {
    pub id: OperandEncodingId,
    pub description: String,
}

/// Full description of an operand.
#[derive(Debug, Deserialize)]
#[must_use]
pub struct Operand {
    pub id: OperandId,
    pub description: String,
}

/// Represents a single instruction variant, e.g.
/// `mov eax, 1`.
#[derive(Debug, Deserialize)]
#[must_use]
pub struct InstructionVariant {
    pub id: u32,
    pub name: Option<String>,
    #[serde(deserialize_with = "crate::custom_deserializers::de_primary_opcode")]
    pub primary_opcode: Vec<u8>,
    pub extended_opcode: Option<u8>,
    pub operand_encoding: OperandEncodingId,
    #[serde(default)]
    pub operands: Vec<OperandId>,
    pub description: String,
    #[serde(default)]
    pub flags: HashSet<FlagId>,
}

impl InstructionVariant {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn get_unique_name(&self) -> String {
        if let Some(name) = self.name.as_ref() {
            return name.clone();
        }

        if self.operands.is_empty() {
            return String::new();
        }

        let operands_capacity = self
            .operands
            .iter()
            .map(|x| <&'static str>::from(x).len())
            .sum::<usize>()
            - 1;
        let mut name = String::with_capacity(operands_capacity);
        let mut operands_iter = self.operands.iter().map(<&'static str>::from);
        let first = operands_iter.next().unwrap();
        name.push_str(first);

        for operand in operands_iter {
            name.push('_');
            name.push_str(operand);
        }
        name
    }
}

/// Represents a group of instruction variants. Typically
/// a group covers a single mnemonic, e.g. `mov`.
#[derive(Debug, Deserialize)]
#[must_use]
pub struct InstructionGroup {
    pub id: u32,
    pub name: String,
    pub variants: Vec<InstructionVariant>,
}

/// Represents the `X86_64` instruction set documentation.
#[derive(Debug, Deserialize)]
#[must_use]
pub struct X86Doc {
    pub operand_encodings: Vec<OperandEncoding>,
    pub operands: Vec<Operand>,
    pub flags: Vec<FlagWithDescription>,
    pub instruction_groups: Vec<InstructionGroup>,
}

const X86_STR: &str = include_str!("../../../../docs/x86.yaml");

impl X86Doc {
    /// Creates a new [`X86Doc`]. This function performs
    /// some parsing and validation internally.
    ///
    /// # Panics
    ///
    /// Whenever the document is invalid internally. This should never happen.
    #[inline(never)]
    pub fn create() -> Self {
        let result: Self = ::serde_saphyr::from_str(X86_STR).expect("Failed to parse X86 document");
        validate(&result);
        result
    }
}
