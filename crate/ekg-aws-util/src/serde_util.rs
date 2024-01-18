use serde::{Deserialize, Serializer};

pub fn serialize_format<S>(
    format: &aws_sdk_neptunedata::types::Format,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format.as_str())
}

/// Deserialize a type `S` by deserializing a string, then using the `FromStr`
/// impl of `S` to create the result. The generic type `S` is not required to
/// implement `Deserialize`.
pub fn deserialize_format_from_str<'de, D>(
    deserializer: D,
) -> Result<aws_sdk_neptunedata::types::Format, D::Error>
where D: serde::Deserializer<'de> {
    let s: String = Deserialize::deserialize(deserializer)?;
    aws_sdk_neptunedata::types::Format::try_parse(s.as_str()).map_err(serde::de::Error::custom)
}
