use crate::{
    globals::ENCODING_OUT_DIR,
    models::{Instruction, InstructionSet, OperandEncoding, OperandKind, Variant, VariantProperty},
    rustfmt,
};

pub fn generate_enc(instruction_set: &InstructionSet) {
    let universal_output_path = ENCODING_OUT_DIR.join("universal_gen.rs");
    let bit32_output_path = ENCODING_OUT_DIR.join("bit32_gen.rs");
    let bit64_output_path = ENCODING_OUT_DIR.join("bit64_gen.rs");

    println!("cargo::rerun-if-changed={}", universal_output_path.to_str().unwrap());
    println!("cargo::rerun-if-changed={}", bit32_output_path.to_str().unwrap());
    println!("cargo::rerun-if-changed={}", bit64_output_path.to_str().unwrap());

    let header = crate::globals::AUTO_GENERATED_HEADER.as_str();
    let prelude = "
#![allow(non_snake_case)]
#![allow(unused_imports)]

use crate::{{
    models::{{EncodedX86Instruction, Immediate8, Immediate16, Immediate32}},
}};
use crate::encoding::utils;
"
    .trim();

    let (universal, bit32, bit64) = generate_content(instruction_set);

    let universal_header = format!(
        "
{header}
//!
//! # Content
//! 
//! Holds encoders for instructions valid in both 32 and 64-bit modes.
{prelude}"
    );

    let bit32_header = format!(
        "
{header}
//!
//! # Content
//! 
//! Holds encoders for instructions valid in 32-bit mode only.
{prelude}"
    );

    let bit64_header = format!(
        "
{header}
//!
//! # Content
//! 
//! Holds encoders for instructions valid in 64-bit mode only.
{prelude}"
    );

    let universal_output = format!("{universal_header}\n\n{universal}");
    let bit32_output = format!("{bit32_header}\n\n{bit32}");
    let bit64_output = format!("{bit64_header}\n\n{bit64}");

    let universal_output = rustfmt::prettify(universal_output);
    let bit32_output = rustfmt::prettify(bit32_output);
    let bit64_output = rustfmt::prettify(bit64_output);

    std::fs::write(universal_output_path, universal_output).unwrap();
    std::fs::write(bit32_output_path, bit32_output).unwrap();
    std::fs::write(bit64_output_path, bit64_output).unwrap();
}

fn generate_content(instruction_set: &InstructionSet) -> (String, String, String) {
    let mut universal = String::new();
    let mut bit32 = String::new();
    let mut bit64 = String::new();

    for instruction in instruction_set.instructions.iter() {
        for variant in instruction.variant_set.variants.iter() {
            if variant.additional_properties.contains(VariantProperty::Bit64Only)
                && variant.additional_properties.contains(VariantProperty::Bit32Only)
            {
                panic!(
                    "Both Bit64Only and Bit32Only are set on mnemonic: {:?}. This is not allowed.",
                    instruction.mnemonic
                );
            }

            let output = if variant.additional_properties.contains(VariantProperty::Bit64Only) {
                &mut bit64
            } else if variant.additional_properties.contains(VariantProperty::Bit32Only) {
                &mut bit32
            } else {
                &mut universal
            };

            output.push_str(&generate_variant(instruction, variant));
        }
    }

    (universal, bit32, bit64)
}

fn generate_variant(instruction: &Instruction, variant: &Variant) -> String {
    match variant.encoding {
        OperandEncoding::ZO => generate_variant_zo(instruction, variant),
        OperandEncoding::I => generate_variant_i(instruction, variant),
        OperandEncoding::MI => generate_variant_mi(instruction, variant),
        OperandEncoding::MR => generate_variant_mr(instruction, variant),
        OperandEncoding::RM => generate_variant_rm(instruction, variant),
    }
}

fn generate_variant_zo(instruction: &Instruction, variant: &Variant) -> String {
    let opcode = to_hex(&variant.opcode);
    let name = function_name(instruction, variant);
    let description = &variant.description;

    format!(
        "
/// {description}
#[inline(always)]
pub const fn {name}() -> EncodedX86Instruction {{
    unsafe {{ EncodedX86Instruction::from_array([{opcode}]) }}
}}
"
    )
}

fn generate_variant_i(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operand_sequence.operands.len() != 1 {
        panic!(
            "Encoding I supports only one operand. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    let opcode = to_hex(&variant.opcode);

    let mut name = function_name(instruction, variant);
    let description = &variant.description;

    let has_oso_prefix = variant.additional_properties.contains(VariantProperty::PrefixOSO);
    let has_rex_w_prefix = variant.additional_properties.contains(VariantProperty::PrefixRexW);

    let operand = &variant.operand_sequence.operands[0];

    let suffix = format!("_{:?}", operand.kind);
    name.push_str(suffix.as_str());

    let (arg, call) = match operand.kind {
        OperandKind::imm8 => {
            let arg = "imm8: Immediate8";
            let call = format!("unsafe {{ utils::enc_I::encode_I_imm8([{opcode}], imm8) }}");
            (arg, call)
        }
        OperandKind::imm16 => {
            let arg = "imm16: Immediate16";
            let call = if has_oso_prefix {
                format!("unsafe {{ utils::enc_I::encode_I_imm16_operand_size_override([{opcode}], imm16) }}")
            } else {
                format!("unsafe {{ utils::enc_I::encode_I_imm16([{opcode}], imm16) }}")
            };
            (arg, call)
        }
        OperandKind::imm32 => {
            let arg = "imm32: Immediate32";
            let call = if has_rex_w_prefix {
                format!("unsafe {{ utils::enc_I::encode_I_imm32_prefix_rex_w([{opcode}], imm32) }}")
            } else {
                format!("unsafe {{ utils::enc_I::encode_I_imm32([{opcode}], imm32) }}")
            };
            (arg, call)
        }
        _ => panic!(
            "Encoding I supports only imm8, imm16, imm32 operands. Not the case for {:?} for a variant in mnemonic: {:?}",
            operand.kind, instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
#[inline(always)]
pub const fn {name}({arg}) -> EncodedX86Instruction {{
    {call}
}}
"
    )
}

fn generate_variant_mi(instruction: &Instruction, variant: &Variant) -> String {
    String::new()
}

fn generate_variant_mr(instruction: &Instruction, variant: &Variant) -> String {
    String::new()
}

fn generate_variant_rm(instruction: &Instruction, variant: &Variant) -> String {
    String::new()
}

fn to_hex(value: &[u8]) -> String {
    value
        .iter()
        .map(|b| format!("0x{:02X}", b))
        .collect::<Vec<_>>()
        .join(",")
}

fn function_name(instruction: &Instruction, variant: &Variant) -> String {
    if variant.name.is_empty() {
        format!("encode_{}", instruction.mnemonic)
    } else {
        format!("encode_{}_{}", instruction.mnemonic, variant.name)
    }
}
