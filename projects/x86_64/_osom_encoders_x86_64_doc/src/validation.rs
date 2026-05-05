use std::collections::HashSet;

use strum::IntoEnumIterator as _;

use crate::FlagId;

use super::{OperandEncodingId, OperandId, X86Doc};

pub fn validate(doc: &X86Doc) {
    // Validate operands.
    let mut doc_operands = HashSet::new();
    let mut doc_operands_count = 0;
    for doc_operand in &doc.operands {
        doc_operands.insert(doc_operand.id.clone());
        doc_operands_count += 1;
    }

    let mut expected_operands_count = 0;
    for operand in OperandId::iter() {
        assert!(doc_operands.contains(&operand), "Operand [{operand:?}] is not valid.");
        expected_operands_count += 1;
    }
    assert_eq!(
        doc_operands_count, expected_operands_count,
        "Expected {expected_operands_count} operands, but got {doc_operands_count}."
    );

    // Validate flags
    let mut flags = HashSet::new();
    let mut flags_count = 0;
    for flag in &doc.flags {
        flags.insert(flag.id.clone());
        flags_count += 1;
    }

    let mut expected_flags_count = 0;
    for flag_id in FlagId::iter() {
        assert!(flags.contains(&flag_id), "Flag [{flag_id:?}] is not valid.");
        expected_flags_count += 1;
    }

    assert_eq!(
        flags_count, expected_flags_count,
        "Expected {expected_flags_count}, but got {flags_count}."
    );

    // Validate operand encodings.
    let mut doc_operand_encodings = HashSet::new();
    let mut doc_operand_encodings_count = 0;
    for doc_operand_encoding in &doc.operand_encodings {
        doc_operand_encodings.insert(doc_operand_encoding.id.clone());
        doc_operand_encodings_count += 1;
    }

    let mut expected_operand_encodings_count = 0;
    for operand_encoding in OperandEncodingId::iter() {
        assert!(
            doc_operand_encodings.contains(&operand_encoding),
            "Operand encoding [{operand_encoding:?}] is not valid."
        );
        expected_operand_encodings_count += 1;
    }
    assert_eq!(
        doc_operand_encodings_count, expected_operand_encodings_count,
        "Expected {expected_operand_encodings_count} operand encodings, but got {doc_operand_encodings_count}."
    );

    // Validate instruction groups.
    let mut seen_names = HashSet::new();
    let mut seen_ids = HashSet::new();
    for doc_instruction_group in &doc.instruction_groups {
        let name = &doc_instruction_group.name;
        assert!(
            valid_name(name),
            "Instruction group name [{name}] contains invalid characters."
        );
        let id = doc_instruction_group.id;
        assert!(
            seen_names.insert(name.clone()),
            "Instruction group name [{name}] is not unique."
        );
        assert!(
            seen_ids.insert(id),
            "Instruction group id [{id}] in group [{name}] is not unique."
        );

        assert!(
            !doc_instruction_group.variants.is_empty(),
            "Instruction group [{name}] must have at least one variant."
        );

        // Validate group variants.
        let mut seen_variant_ids = HashSet::new();
        let mut seen_variant_names = HashSet::new();
        for doc_instruction_variant in &doc_instruction_group.variants {
            let variant_id = doc_instruction_variant.id;
            assert!(
                seen_variant_ids.insert(variant_id),
                "Instruction variant id [{variant_id}] inside group [{name}] is not unique."
            );
            let unique_name = doc_instruction_variant.get_unique_name();
            assert!(
                seen_variant_names.insert(unique_name.clone()),
                "Instruction variant id [{variant_id}] inside group [{name}] has unique name [{unique_name}] which is not unique."
            );

            let opcode = &doc_instruction_variant.primary_opcode;
            if doc_instruction_variant.operand_encoding == OperandEncodingId::OI {
                assert!(opcode.len() == 1, "Opcode for OI encoding must be 1 byte long.");
            }
        }
    }
}

fn valid_name(name: &str) -> bool {
    let mut first_char = true;
    !name.is_empty()
        && name.chars().all(|c| {
            if first_char {
                first_char = false;
                return c.is_ascii_alphabetic();
            }
            c.is_ascii_alphanumeric() || c == '_'
        })
}
