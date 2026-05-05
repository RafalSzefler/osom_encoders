use serde::Deserialize;

pub fn de_primary_opcode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.len() % 2 != 0 {
        return Err(serde::de::Error::custom(
            "Primary opcode must be valid hexadecimal string.",
        ));
    }

    if s.len() > 8 {
        return Err(serde::de::Error::custom(
            "Primary opcode must be less than 8 characters.",
        ));
    }

    if s.len() < 2 {
        return Err(serde::de::Error::custom(
            "Primary opcode must be at least 2 characters.",
        ));
    }

    if !s.chars().all(|c| c.is_ascii_digit() || c.is_uppercase()) {
        return Err(serde::de::Error::custom(
            "Primary opcode must be in uppercase hexadecimal format.",
        ));
    }

    let Ok(decoded) = hex::decode(s) else {
        return Err(serde::de::Error::custom("Failed to decode primary opcode."));
    };

    Ok(decoded)
}
