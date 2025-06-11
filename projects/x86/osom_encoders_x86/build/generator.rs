use osom_encoders_x86_ref_models::models::X86Reference;


#[derive(Debug)]
pub enum GenerateCodeError {
    Io(std::io::Error),
}

impl From<std::io::Error> for GenerateCodeError {
    fn from(error: std::io::Error) -> Self {
        GenerateCodeError::Io(error)
    }
}

pub fn generate_code(x86_reference: &X86Reference) -> Result<String, GenerateCodeError> {
    todo!()
}