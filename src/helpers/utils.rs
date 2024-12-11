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


#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(deserialize_with = "parse_f64_or_skip", 
                serialize_with = "serialize_f64")]
        value: f64,
    }

    #[test]
    fn test_deserialize_f64() {
        let json = r#"{"value": 42.5}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 42.5);
    }

    #[test]
    fn test_deserialize_string() {
        let json = r#"{"value": "42.5"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 42.5);
    }

    #[test]
    fn test_serialize_f64() {
        let test_struct = TestStruct { value: 42.5 };
        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"value":42.5}"#);
    }

    #[test]
    fn test_deserialize_invalid_string() {
        let json = r#"{"value": "not a number"}"#;
        let result = serde_json::from_str::<TestStruct>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_zero() {
        let json = r#"{"value": 0}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 0.0);
    }

    #[test]
    fn test_deserialize_negative() {
        let json = r#"{"value": "-42.5"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, -42.5);
    }

    #[test]
    fn test_scientific_notation() {
        let json = r#"{"value": "1.23e-4"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 0.000123);
    }

    #[test]
    fn test_serialize_various_values() {
        let test_cases = vec![
            0.0,
            42.5,
            -123.456,
            1e-10,
            f64::MAX,
            f64::MIN_POSITIVE,
        ];

        for val in test_cases {
            let test_struct = TestStruct { value: val };
            let json = serde_json::to_string(&test_struct).unwrap();
            let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.value, val);
        }
    }
}