use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct MongoId(String);

impl std::convert::TryFrom<String> for MongoId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_mongo_regex(&value)
    }
}

// Or implement deserialize and serialize manually

// impl<'de> Deserialize<'de> for MongoId {
//     fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//         let value = String::deserialize(deserializer)?;

//         validate_mongo_regex(&value).map_err(serde::de::Error::custom)
//     }
// }

fn validate_mongo_regex(value: &str) -> Result<MongoId, String> {
    static INSTANCE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let regex = INSTANCE.get_or_init(|| regex::Regex::new(r"^[a-fA-F0-9]{24}$").unwrap());

    if regex.is_match(value) {
        Ok(MongoId(value.to_string()))
    } else {
        Err(format!("Invalid MongoId, must follow the regex: {regex}"))
    }
}
