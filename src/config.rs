use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
pub struct Config {
    from_file: Option<Vec<String>>,
    pub template_name: String,
    pub context: JsonValue,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::from_str(&std::fs::read_to_string(path).ok()?)
    }

    pub fn from_str(input: &str) -> Option<Self> {
        let mut s: Self = serde_json::from_str(input).ok()?;

        if let Some(fields) = &s.from_file {
            for field_name in fields {
                let field = s.context.pointer(field_name).unwrap();

                let filename = field.as_str();

                if let Some(filename) = filename {
                    *s.context.pointer_mut(field_name).unwrap() =
                        json!(std::fs::read_to_string(filename).ok()?);
                } else {
                    return None;
                }
            }
        }

        Some(s)
    }
}
