use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;

pub enum ConfigError {
    File,
    Parse,
    MissingField,
    ReplaceContent,
    Type,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    from_file: Option<Vec<String>>,
    pub template_name: String,
    pub context: JsonValue,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        Self::from_str(match &std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return Err(ConfigError::File),
        })
    }

    pub fn from_str(input: &str) -> Result<Self, ConfigError> {
        let mut s: Self = match serde_json::from_str(input) {
            Ok(c) => c,
            Err(_) => return Err(ConfigError::Parse),
        };

        if let Some(fields) = &s.from_file {
            for field_name in fields {
                let field = match s.context.pointer_mut(field_name) {
                    Some(f) => f,
                    None => return Err(ConfigError::MissingField),
                };

                let filename = field.as_str();

                if let Some(filename) = filename {
                    *field = json!(match std::fs::read_to_string(filename) {
                        Ok(s) => s,
                        Err(_) => return Err(ConfigError::ReplaceContent),
                    });
                } else {
                    return Err(ConfigError::Type);
                }
            }
        }

        Ok(s)
    }
}
