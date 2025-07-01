use std::path::Path;

use crate::models::InstructionSet;

mod enc;
mod mnemonics;

pub fn generate(instruction_set: &InstructionSet, output_dir: &Path) {
    mnemonics::generate_mnemonics_enum(instruction_set, output_dir);
    enc::generate_enc(instruction_set, output_dir);
}
