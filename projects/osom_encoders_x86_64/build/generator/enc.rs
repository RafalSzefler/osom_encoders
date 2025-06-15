use crate::{
    globals::ENCODING_OUT_DIR,
    models::{Instruction, InstructionSet, Operand, OperandEncoding, Variant, VariantProperty},
    rustfmt,
};

pub fn generate_enc(instruction_set: &InstructionSet) {
    let enc_output_path = ENCODING_OUT_DIR.join("enc_gen.rs");

    println!("cargo::rerun-if-changed={}", enc_output_path.to_str().unwrap());

    let header = crate::globals::AUTO_GENERATED_HEADER.as_str();
    let prelude = "
#![allow(non_snake_case)]
#![allow(unused_imports)]

use crate::{{
    models::{{GPROrMemory, EncodedX86_64Instruction, Immediate8, Immediate16, Immediate32}},
}};
use crate::encoding::utils;
"
    .trim();

    let enc_content = generate_content(instruction_set);

    let enc_header = format!(
        "
{header}
//!
//! # Content
//! 
//! Holds encoders for instructions valid in both 32 and 64-bit modes.
{prelude}"
    );

    let enc_output = format!("{enc_header}\n\n{enc_content}");
    let enc_output = rustfmt::prettify(enc_output);

    std::fs::write(enc_output_path, enc_output).unwrap();
}

fn generate_content(instruction_set: &InstructionSet) -> String {
    let mut output = String::new();

    for instruction in instruction_set.instructions.iter() {
        for variant in instruction.variant_set.variants.iter() {
            output.push_str(&generate_variant(instruction, variant));
        }
    }

    output
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
pub const fn {name}() -> EncodedX86_64Instruction {{
    unsafe {{ EncodedX86_64Instruction::from_array([{opcode}]) }}
}}
"
    )
}

fn generate_variant_i(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operands.len() != 1 {
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

    let operand = &variant.operands[0];

    let suffix = format!("_{:?}", operand);
    name.push_str(suffix.as_str());

    let (arg, call) = match operand {
        Operand::imm8 => {
            let arg = "imm8: Immediate8";
            let call = format!("unsafe {{ utils::enc_I::encode_I_imm8([{opcode}], imm8) }}");
            (arg, call)
        }
        Operand::imm16 => {
            let arg = "imm16: Immediate16";
            let call = if has_oso_prefix {
                format!("unsafe {{ utils::enc_I::encode_I_imm16_operand_size_override([{opcode}], imm16) }}")
            } else {
                format!("unsafe {{ utils::enc_I::encode_I_imm16([{opcode}], imm16) }}")
            };
            (arg, call)
        }
        Operand::imm32 => {
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
            operand, instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
#[inline(always)]
pub const fn {name}({arg}) -> EncodedX86_64Instruction {{
    {call}
}}
"
    )
}

#[allow(unused_variables)]
fn generate_variant_mi(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operands.len() != 2 {
        panic!(
            "Encoding MI supports two operands only. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    let Some(extended_opcode) = variant.extended_opcode else {
        panic!(
            "Encoding MI requires extended opcode. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    };

    let opcode = to_hex(&variant.opcode);
    let extended_opcode = to_hex(&[extended_opcode]);
    let description = &variant.description;

    let rm_operand = &variant.operands[0];
    let imm_operand = &variant.operands[1];

    let name = function_name(instruction, variant);
    let name = format!("{}_{:#?}_{:#?}", name, rm_operand, imm_operand);

    let (args, body) = match (&rm_operand, &imm_operand) {
        (Operand::rm8, Operand::imm8) => {
            let args = "rm8: GPROrMemory, imm8: Immediate8";
            let body =
                format!("unsafe {{ utils::enc_MI::encode_MI_rm8_imm8([{opcode}], {extended_opcode}, rm8, imm8) }}");
            (args, body)
        }
        _ => panic!(
            "Encoding MI requires rm and imm operands to be of the same size. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
#[inline(always)]
pub const fn {name}({args}) -> EncodedX86_64Instruction {{
    {body}
}}
"
    )
}

#[allow(unused_variables)]
fn generate_variant_mr(instruction: &Instruction, variant: &Variant) -> String {
    String::new()
}

#[allow(unused_variables)]
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
