pub mod models;

use std::{io::BufReader, path::Path};
use xsd_parser::quick_xml::{DeserializeSync, IoReader, XmlReader, Error};

#[derive(Debug)]
pub enum ReadX86XMLError {
    Io(std::io::Error),
    Xml(Error),
}

impl From<std::io::Error> for ReadX86XMLError {
    fn from(error: std::io::Error) -> Self {
        ReadX86XMLError::Io(error)
    }
}

impl From<Error> for ReadX86XMLError {
    fn from(error: Error) -> Self {
        ReadX86XMLError::Xml(error)
    }
}

pub fn read_x86_xml(path: &Path) -> Result<models::X86Reference, ReadX86XMLError> {
    let input_file = std::fs::File::open(path)?;
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let x86_reference = models::X86Reference::deserialize(&mut reader)?;
    Ok(x86_reference)
}
