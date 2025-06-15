use std::collections::HashSet;

use crate::{globals::ENCODING_OUT_DIR, models::InstructionSet, rustfmt};

pub fn generate_mnemonics_enum(instruction_set: &InstructionSet) {
    let output_path = ENCODING_OUT_DIR.join("mnemonics_gen.rs");
    println!("cargo::rerun-if-changed={}", output_path.to_str().unwrap());

    let mnemonics = instruction_set
        .instructions
        .iter()
        .map(|instruction| &instruction.mnemonic)
        .collect::<Vec<_>>();

    let distinct_mnemonics = mnemonics.iter().collect::<HashSet<_>>().len();
    if mnemonics.len() != distinct_mnemonics {
        panic!("Each instruction has to have unique mnemonic.");
    }

    let header = crate::globals::AUTO_GENERATED_HEADER.as_str();

    let mut mnemonics_variants = String::new();
    let mut mnemonics_to_str = String::new();
    let mut mnemonics_from_str = String::new();
    for (i, mnemonic) in mnemonics.iter().enumerate() {
        if mnemonic.is_empty() {
            panic!("Each instruction has to have a unique, non-empty mnemonic.");
        }

        let idx = i + 1;
        mnemonics_variants.push_str(&format!("{mnemonic} = {idx},\n"));
        mnemonics_to_str.push_str(&format!("Self::{mnemonic} => \"{mnemonic}\",\n"));
        mnemonics_from_str.push_str(&format!("\"{mnemonic}\" => Ok(Self::{mnemonic}),\n"));
    }

    let output = format!(
        "{header}
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidMnemonicError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[must_use]
pub enum Mnemonic {{
    {mnemonics_variants}
}}

impl Mnemonic {{
    #[inline]
    #[must_use]
    pub const fn as_str(self) -> &'static str {{
        match self {{
            {mnemonics_to_str}
        }}
    }}

    #[inline]
    pub fn from_str(text: &str) -> Result<Self, InvalidMnemonicError> {{
        match text {{
            {mnemonics_from_str}
            _ => Err(InvalidMnemonicError),
        }}
    }}
}}

const _: () = const {{
    assert!(size_of::<InvalidMnemonicError>() == 0, \"InvalidMnemonicError must be 0 bytes.\");
    assert!(size_of::<Mnemonic>() == 1, \"Mnemonic must be 1 byte.\");
    assert!(size_of::<Option<Mnemonic>>() == 1, \"Option<Mnemonic> must be 1 byte.\");

}};
    "
    );

    let output = rustfmt::prettify(output);

    std::fs::write(output_path, output).unwrap();
}
