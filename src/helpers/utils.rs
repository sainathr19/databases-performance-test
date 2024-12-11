use serde::{Deserialize, Deserializer, Serializer};

pub fn parse_f64_or_skip<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum F64OrString {
        F64(f64),
        String(String),
    }

    match F64OrString::deserialize(deserializer)? {
        F64OrString::F64(val) => Ok(val),
        F64OrString::String(s) => s.parse().map_err(serde::de::Error::custom),
    }
}

pub fn serialize_f64<S>(val: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(*val)
}