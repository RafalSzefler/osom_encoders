use osom_encoders_x86::models::EncodedX86Instruction;

pub fn assert_encoded_instruction_eq(expected: &[u8], actual: &EncodedX86Instruction) {
    assert_eq!(expected, actual.as_slice());
}
