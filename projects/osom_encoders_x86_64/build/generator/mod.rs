use crate::models::InstructionSet;

mod enc;
mod mnemonics;

pub fn generate(instruction_set: &InstructionSet) {
    mnemonics::generate_mnemonics_enum(instruction_set);
    enc::generate_enc(instruction_set);
}
