#![allow(clippy::upper_case_acronyms)]

mod instruction_set;
use std::path::Path;

pub use instruction_set::*;

mod instruction_set_deserialize;
#[allow(unused_imports)]
pub use instruction_set_deserialize::*;

mod instruction_set_variant_property;
pub use instruction_set_variant_property::*;

pub fn read_instruction_set(xml_path: &Path) -> InstructionSet {
    let instruction_set_content = std::fs::read_to_string(xml_path).unwrap();
    let instruction_set: InstructionSet = serde_xml_rs::from_str(&instruction_set_content).unwrap();

    instruction_set
}
