#![allow(non_camel_case_types)]

use serde::Deserialize;
use strum::EnumString;

use super::VariantProperty;

#[derive(Debug, Deserialize, EnumString)]
pub enum OperandKind {
    /// Immediate 8-bit.
    imm8,

    /// Immediate 16-bit.
    imm16,

    /// Immediate 32-bit.
    imm32,

    /// Immediate 64-bit.
    imm64,

    /// 8-bit general-purpose register or memory.
    rm8,

    /// 16-bit general-purpose register or memory.
    rm16,

    /// 32-bit general-purpose register or memory.
    rm32,

    /// 64-bit general-purpose register or memory.
    rm64,

    /// 8-bit general-purpose register.
    r8,

    /// 16-bit general-purpose register.
    r16,

    /// 32-bit general-purpose register.
    r32,

    /// 64-bit general-purpose register.
    r64,
}

#[derive(Debug, EnumString)]
pub enum OperandEncoding {
    /// No operands. The "ZO" abbreviation follows Intel manual convention.
    ZO,

    /// Simple immediate encoding.
    I,

    /// Immediate to reg/mem.
    MI,

    /// Reg to reg/mem.
    MR,

    /// Reg/mem to reg.
    RM,
}

#[derive(Debug, Deserialize)]
pub struct Operand {
    #[serde(rename = "#text")]
    pub kind: OperandKind,
}

#[derive(Debug, Deserialize, Default)]
pub struct OperandSequence {
    #[serde(rename = "operand")]
    pub operands: Vec<Operand>,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    #[serde(deserialize_with = "super::deserialize_opcode")]
    pub opcode: Vec<u8>,

    #[serde(deserialize_with = "super::deserialize_alpha_string", default = "String::new")]
    pub name: String,

    pub extended_opcode: Option<u8>,

    #[serde(default = "Default::default")]
    pub operand_sequence: OperandSequence,

    #[serde(rename = "operand_encoding")]
    pub encoding: OperandEncoding,

    #[serde(default = "Default::default")]
    pub additional_properties: VariantProperty,

    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct VariantSet {
    #[serde(rename = "variant")]
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Instruction {
    #[serde(deserialize_with = "super::deserialize_alpha_string")]
    pub mnemonic: String,

    pub variant_set: VariantSet,
}

#[derive(Debug, Deserialize)]
pub struct InstructionSet {
    #[serde(rename = "instruction")]
    pub instructions: Vec<Instruction>,
}
