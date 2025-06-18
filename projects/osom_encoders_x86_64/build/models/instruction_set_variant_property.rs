bitflags::bitflags! {
    #[derive(Debug, Default)]
    pub struct VariantProperty: u8 {
        const None          = 0b00000000;
        const PrefixRex     = 0b00000001;
        const PrefixRexW    = 0b00000010;

        /// Operand size override prefix.
        const PrefixOSO     = 0b00000100;
    }
}

#[derive(Debug)]
pub struct InvalidVariantFlagError;

impl TryFrom<&str> for VariantProperty {
    type Error = InvalidVariantFlagError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "None" | "" => Ok(VariantProperty::None),
            "PrefixRex" => Ok(VariantProperty::PrefixRex),
            "PrefixRexW" => Ok(VariantProperty::PrefixRexW),
            "PrefixOSO" => Ok(VariantProperty::PrefixOSO),
            _ => Err(InvalidVariantFlagError),
        }
    }
}
