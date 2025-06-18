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
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::wildcard_imports)]

use crate::{{
    models::{{GPR, GPROrMemory, Memory, EncodedX86_64Instruction, Immediate8, Immediate16, Immediate32, Immediate64}},
}};
use crate::encoders::utils;
"
    .trim();

    let enc_content = generate_content(instruction_set);

    let enc_header = format!(
        "
{header}
//!
//! # Content
//! 
//! Holds encoders for instructions valid in 64-bit X86 instruction set.
{prelude}"
    );

    let enc_output = format!("{enc_header}\n\n{enc_content}");
    let enc_output = rustfmt::prettify(enc_output);

    std::fs::write(enc_output_path, enc_output).unwrap();
}

fn generate_content(instruction_set: &InstructionSet) -> String {
    let mut output = String::new();

    let mut single_variants = Vec::new();
    let mut multi_variants = Vec::new();

    for instruction in &instruction_set.instructions {
        if instruction.variant_set.variants.len() == 1 {
            single_variants.push(instruction);
        } else {
            multi_variants.push(instruction);
        }
    }

    fn generate_variants(instruction: &Instruction) -> String {
        let mut variants = String::new();
        for variant in instruction.variant_set.variants.iter() {
            variants.push_str(&generate_variant(instruction, variant));
        }
        variants
    }

    let mut single_variants_submodule =
        "/// Holds encoders for instructions that have only one variant.\npub mod singleton { use super::*;\n\n"
            .to_string();
    for instruction in single_variants {
        let variants = generate_variants(instruction);
        single_variants_submodule.push_str(&variants);
    }
    single_variants_submodule.push_str("}\n\n");
    output.push_str(&single_variants_submodule);

    for instruction in multi_variants {
        let variants = generate_variants(instruction);
        let submodule = format!(
            "/// Holds encoders for variants of `{mnemonic}` instruction.\npub mod {mnemonic} {{ use super::*;\n\n{variants} }}\n\n",
            mnemonic = instruction.mnemonic
        );
        output.push_str(&submodule);
    }

    output
}

fn generate_variant(instruction: &Instruction, variant: &Variant) -> String {
    match variant.encoding {
        OperandEncoding::ZO => generate_variant_zo(instruction, variant),
        OperandEncoding::I => generate_variant_i(instruction, variant),
        OperandEncoding::MI => generate_variant_mi(instruction, variant),
        OperandEncoding::MR => generate_variant_mr(instruction, variant),
        OperandEncoding::O => generate_variant_o(instruction, variant),
        OperandEncoding::OI => generate_variant_oi(instruction, variant),
        OperandEncoding::M => generate_variant_m(instruction, variant),
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
                format!("unsafe {{ utils::enc_MI::encode_MI_rm8_imm8([{opcode}], {extended_opcode}, &rm8, imm8) }}");
            (args, body)
        }
        (Operand::rm16, Operand::imm16) => {
            let args = "rm16: GPROrMemory, imm16: Immediate16";
            let body = format!(
                "unsafe {{ utils::enc_MI::encode_MI_rm16_imm16([{opcode}], {extended_opcode}, &rm16, imm16) }}"
            );
            (args, body)
        }
        (Operand::rm32, Operand::imm32) => {
            let args = "rm32: GPROrMemory, imm32: Immediate32";
            let body = format!(
                "unsafe {{ utils::enc_MI::encode_MI_rm32_imm32([{opcode}], {extended_opcode}, &rm32, imm32) }}"
            );
            (args, body)
        }
        (Operand::rm64, Operand::imm32) => {
            let args = "rm64: GPROrMemory, imm32: Immediate32";
            let body = format!(
                "unsafe {{ utils::enc_MI::encode_MI_rm64_imm32([{opcode}], {extended_opcode}, &rm64, imm32) }}"
            );
            (args, body)
        }
        (Operand::rm16, Operand::imm8) => {
            let args = "rm16: GPROrMemory, imm8: Immediate8";
            let body =
                format!("unsafe {{ utils::enc_MI::encode_MI_rm16_imm8([{opcode}], {extended_opcode}, &rm16, imm8) }}");
            (args, body)
        }
        (Operand::rm32, Operand::imm8) => {
            let args = "rm32: GPROrMemory, imm8: Immediate8";
            let body =
                format!("unsafe {{ utils::enc_MI::encode_MI_rm32_imm8([{opcode}], {extended_opcode}, &rm32, imm8) }}");
            (args, body)
        }
        (Operand::rm64, Operand::imm8) => {
            let args = "rm64: GPROrMemory, imm8: Immediate8";
            let body =
                format!("unsafe {{ utils::enc_MI::encode_MI_rm64_imm8([{opcode}], {extended_opcode}, &rm64, imm8) }}");
            (args, body)
        }
        _ => panic!(
            "Encoding MI requires rm and imm operands to be of appropriate size. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
///
/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
#[inline(always)]
pub const unsafe fn {name}({args}) -> EncodedX86_64Instruction {{
    {body}
}}
"
    )
}

fn generate_variant_mr(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operands.len() != 2 {
        panic!(
            "Encoding MR supports two operands only. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    if variant.extended_opcode.is_some() {
        panic!(
            "Encoding MR doesn't use extended opcode. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    };

    let opcode = to_hex(&variant.opcode);
    let description = &variant.description;

    let first_operand = &variant.operands[0];
    let second_operand = &variant.operands[1];

    fn is_memory_operand(operand: &Operand) -> bool {
        matches!(operand, Operand::rm8 | Operand::rm16 | Operand::rm32 | Operand::rm64)
    }

    if is_memory_operand(first_operand) && is_memory_operand(second_operand) {
        panic!(
            "Encoding MR requires one operand to be a register and the other to be a memory operand. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    fn operand_to_arg(operand: &Operand) -> &'static str {
        match operand {
            Operand::rm8 => "rm8: GPROrMemory",
            Operand::rm16 => "rm16: GPROrMemory",
            Operand::rm32 => "rm32: GPROrMemory",
            Operand::rm64 => "rm64: GPROrMemory",
            Operand::reg8 => "reg8: GPR",
            Operand::reg16 => "reg16: GPR",
            Operand::reg32 => "reg32: GPR",
            Operand::reg64 => "reg64: GPR",
            Operand::m => "m: Memory",
            _ => panic!("Operand {:?} is not supported for MR encoding", operand),
        }
    }

    fn operant_to_arg_name(operand: &Operand) -> &'static str {
        match operand {
            Operand::rm8 => "rm8",
            Operand::rm16 => "rm16",
            Operand::rm32 => "rm32",
            Operand::rm64 => "rm64",
            Operand::reg8 => "reg8",
            Operand::reg16 => "reg16",
            Operand::reg32 => "reg32",
            Operand::reg64 => "reg64",
            Operand::m => "m",
            _ => panic!("Operand {:?} is not supported for MR encoding", operand),
        }
    }

    let first_full_arg = operand_to_arg(first_operand);
    let second_full_arg = operand_to_arg(second_operand);

    let mut first_arg_name = operant_to_arg_name(first_operand);
    let mut second_arg_name = operant_to_arg_name(second_operand);
    if is_memory_operand(second_operand) {
        std::mem::swap(&mut first_arg_name, &mut second_arg_name);
    }

    let args = format!("{first_full_arg}, {second_full_arg}");
    let body = if second_arg_name == "m" {
        format!("unsafe {{ utils::enc_MR::encode_MR_m([{opcode}], &{second_arg_name}, {first_arg_name}) }}")
    } else {
        format!("unsafe {{ utils::enc_MR::encode_MR([{opcode}], &{first_arg_name}, {second_arg_name}) }}")
    };

    let name = function_name(instruction, variant);
    let name = format!("{}_{:#?}_{:#?}", name, first_operand, second_operand);

    format!(
        "
/// {description}
///
/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
#[inline(always)]
pub const unsafe fn {name}({args}) -> EncodedX86_64Instruction {{
    {body}
}}
"
    )
}

fn generate_variant_oi(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operands.len() != 2 {
        panic!(
            "Encoding OI supports two operands only. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    if variant.extended_opcode.is_some() {
        panic!(
            "Encoding OI doesn't use extended opcode. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    };

    let opcode = to_hex(&variant.opcode);
    let description = &variant.description;

    let rm_operand = &variant.operands[0];
    let imm_operand = &variant.operands[1];

    let name = function_name(instruction, variant);
    let name = format!("{}_{:#?}_{:#?}", name, rm_operand, imm_operand);

    let (args, body) = match (&rm_operand, &imm_operand) {
        (Operand::reg8, Operand::imm8) => {
            let args = "reg8: GPR, imm8: Immediate8";
            let body = format!("unsafe {{ utils::enc_OI::encode_OI_r8_imm8({opcode}, reg8, imm8) }}");
            (args, body)
        }
        (Operand::reg16, Operand::imm16) => {
            let args = "reg16: GPR, imm16: Immediate16";
            let body = format!("unsafe {{ utils::enc_OI::encode_OI_r16_imm16({opcode}, reg16, imm16) }}");
            (args, body)
        }
        (Operand::reg32, Operand::imm32) => {
            let args = "reg32: GPR, imm32: Immediate32";
            let body = format!("unsafe {{ utils::enc_OI::encode_OI_r32_imm32({opcode}, reg32, imm32) }}");
            (args, body)
        }
        (Operand::reg64, Operand::imm64) => {
            let args = "reg64: GPR, imm64: Immediate64";
            let body = format!("unsafe {{ utils::enc_OI::encode_OI_r64_imm64({opcode}, reg64, imm64) }}");
            (args, body)
        }
        _ => panic!(
            "Encoding OI requires reg and imm operands to be of appropriate size. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
///
/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
#[inline(always)]
pub const unsafe fn {name}({args}) -> EncodedX86_64Instruction {{
    {body}
}}
"
    )
}

fn generate_variant_m(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operands.len() != 1 {
        panic!(
            "Encoding M supports single operand only. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    let Some(extended_opcode) = variant.extended_opcode else {
        panic!(
            "Encoding M requires extended opcode. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    };

    let extended_opcode = to_hex(&[extended_opcode]);

    let opcode = to_hex(&variant.opcode);
    let description = &variant.description;

    let has_rex_w_prefix = if variant.additional_properties.contains(VariantProperty::PrefixRexW) {
        "true"
    } else {
        "false"
    };

    let has_oso_prefix = if variant.additional_properties.contains(VariantProperty::PrefixOSO) {
        "true"
    } else {
        "false"
    };

    let rm_operand = &variant.operands[0];

    let name = function_name(instruction, variant);
    let name = format!("{}_{:#?}", name, rm_operand);

    let (args, body) = match &rm_operand {
        Operand::rm16 => {
            let args = "rm16: GPROrMemory";
            let body = format!(
                "unsafe {{ utils::enc_M::encode_M_gpr_or_memory([{opcode}], {extended_opcode}, &rm16, {has_rex_w_prefix}, {has_oso_prefix}) }}"
            );
            (args, body)
        }
        Operand::rm64 => {
            let args = "rm64: GPROrMemory";
            let body = format!(
                "unsafe {{ utils::enc_M::encode_M_gpr_or_memory([{opcode}], {extended_opcode}, &rm64, {has_rex_w_prefix}, {has_oso_prefix}) }}"
            );
            (args, body)
        }
        _ => panic!(
            "Encoding M requires rm operand and of appropriate size. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
///
/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
#[inline(always)]
pub const unsafe fn {name}({args}) -> EncodedX86_64Instruction {{
    {body}
}}
"
    )
}

fn generate_variant_o(instruction: &Instruction, variant: &Variant) -> String {
    if variant.operands.len() != 1 {
        panic!(
            "Encoding O supports single reg operand only. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    }

    if variant.extended_opcode.is_some() {
        panic!(
            "Encoding O doesn't use extended opcode. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        );
    };

    let opcode = to_hex(&variant.opcode);
    let description = &variant.description;

    let reg_operand = &variant.operands[0];

    let name = function_name(instruction, variant);
    let name = format!("{}_{:#?}", name, reg_operand);

    let (args, body) = match &reg_operand {
        Operand::reg16 => {
            let args = "reg16: GPR";
            let body = format!("unsafe {{ utils::enc_O::encode_O({opcode}, reg16) }}");
            (args, body)
        }
        Operand::reg64 => {
            let args = "reg64: GPR";
            let body = format!("unsafe {{ utils::enc_O::encode_O({opcode}, reg64) }}");
            (args, body)
        }
        _ => panic!(
            "Encoding O supports only reg16 and reg64 operands. Not the case for a variant in mnemonic: {:?}",
            instruction.mnemonic
        ),
    };

    format!(
        "
/// {description}
///
/// # Safety
///
/// The caller has to ensure that the operands are valid,
/// in particular the function does not check register sizes.
#[inline(always)]
pub const unsafe fn {name}({args}) -> EncodedX86_64Instruction {{
    {body}
}}
"
    )
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
