use _osom_encoders_x86_64_doc::{OperandEncodingId, OperandId, X86Doc};

#[test]
fn test_x86_doc() {
    let x86_doc = X86Doc::create();
    assert_eq!(x86_doc.operand_encodings.len(), 6);
    assert_eq!(x86_doc.operands.len(), 13);
    assert_eq!(x86_doc.instruction_groups.len(), 13);

    let lock_group = x86_doc
        .instruction_groups
        .iter()
        .find(|group| group.name == "lock")
        .unwrap();
    assert_eq!(lock_group.variants.len(), 1);
    let lock_variant = lock_group.variants.iter().next().unwrap();
    assert_eq!(lock_variant.get_unique_name(), "");
    assert_eq!(lock_variant.primary_opcode, vec![0xF0]);
    assert_eq!(lock_variant.operand_encoding, OperandEncodingId::ZO);
    assert_eq!(lock_variant.operands, vec![]);

    let mov_group = x86_doc
        .instruction_groups
        .iter()
        .find(|group| group.name == "mov")
        .unwrap();
    let mov_variant_2 = mov_group.variants.iter().find(|variant| variant.id == 1).unwrap();
    assert_eq!(mov_variant_2.get_unique_name(), "RM16_Imm16");
    assert_eq!(mov_variant_2.primary_opcode, &[0xC7]);
    assert_eq!(mov_variant_2.extended_opcode, Some(0));
    assert_eq!(mov_variant_2.operand_encoding, OperandEncodingId::MI);
    assert_eq!(mov_variant_2.operands, vec![OperandId::RM16, OperandId::Imm16]);
}
