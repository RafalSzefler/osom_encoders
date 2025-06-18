use serde::{Deserialize, Deserializer};

use crate::models::Operand;

use super::{OperandEncoding, VariantProperty};

impl<'de> Deserialize<'de> for VariantProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        {
            let text = String::deserialize(deserializer)?;
            let s = text.trim();
            let mut result = VariantProperty::None;

            for piece in s.split(',') {
                let piece = piece.trim();
                match VariantProperty::try_from(piece) {
                    Ok(flag) => result |= flag,
                    Err(_) => return Err(serde::de::Error::custom(format!("Invalid flag: {}", piece))),
                }
            }
            Ok(result)
        }
    }
}

impl<'de> Deserialize<'de> for OperandEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let s = text.trim();
        match OperandEncoding::try_from(s) {
            Ok(encoding) => Ok(encoding),
            Err(_) => Err(serde::de::Error::custom(format!("Invalid operand encoding: {}", s))),
        }
    }
}

pub(super) fn deserialize_opcode<'de, D>(value: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(value)?;
    let s = text.trim();

    let total_opcode = s.split('+').collect::<Vec<_>>();
    let total_opcode_len = total_opcode.len();

    let mut result = Vec::with_capacity(4);

    let opcode = if total_opcode_len > 1 {
        for prefix in total_opcode.iter().take(total_opcode_len - 2) {
            let prefix = prefix.trim();
            match prefix {
                "O" => result.push(0x66),     // Operand size override prefix
                "REX.W" => result.push(0x48), // REX.W prefix
                _ => return Err(serde::de::Error::custom(format!("Invalid opcode prefix: {}", prefix))),
            }
        }
        total_opcode[total_opcode_len - 1].trim()
    } else {
        total_opcode[0].trim()
    };

    let len = opcode.len();

    if len % 2 != 0 {
        return Err(serde::de::Error::custom(format!(
            "Invalid opcode hex: length must be even, got {}",
            len
        )));
    }

    if len > 8 {
        return Err(serde::de::Error::custom(format!(
            "Invalid opcode hex: length must be at most 8, got {}",
            len
        )));
    }
    let mut idx = 0;
    while idx < len {
        let view = &opcode[idx..=idx + 1];
        let byte = u8::from_str_radix(view, 16)
            .map_err(|_| serde::de::Error::custom(format!("Invalid opcode hex: {}", view)))?;
        result.push(byte);
        idx += 2;
    }

    Ok(result)
}

pub(super) fn deserialize_alpha_string<'de, D>(value: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(value)?;
    let s = text.trim();

    if s.is_empty() {
        return Ok(String::new());
    }

    let mut chars = s.chars();
    let next = chars.next().unwrap();
    if !next.is_alphabetic() {
        return Err(serde::de::Error::custom(format!("Invalid alpha string: {}", s)));
    }

    for chr in chars {
        if !chr.is_alphanumeric() {
            return Err(serde::de::Error::custom(format!("Invalid alpha string: {}", s)));
        }
    }

    Ok(s.to_string())
}

pub(super) fn deserialize_operands<'de, D>(value: D) -> Result<Vec<Operand>, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(value)?;
    let pieces = text.split(',').map(|s| s.trim()).collect::<Vec<_>>();
    if pieces.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = Vec::with_capacity(pieces.len());
    for piece in pieces {
        match Operand::try_from(piece) {
            Ok(operand) => result.push(operand),
            Err(_) => return Err(serde::de::Error::custom(format!("Invalid operand name: {}", piece))),
        }
    }

    Ok(result)
}
