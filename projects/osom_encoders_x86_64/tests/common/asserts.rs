use osom_encoders_x86_64::models::EncodedX86_64Instruction;

pub fn assert_encoded_instruction_eq(expected: &[u8], actual: &EncodedX86_64Instruction) {
    assert_hex::assert_eq_hex!(expected, actual.as_slice());
}
