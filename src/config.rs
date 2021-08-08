use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub enum ConfigError {
    NotFound { name: String },
    JsonParseError { context: String },
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    from_file: Option<Vec<String>>,
    pub template_name: String,
    pub context: JsonValue,
}

impl Config {
    pub fn from_file(path: &dyn AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        Self::from_str(match &std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                return Err(ConfigError::NotFound {
                    name: format!("Path: {}", path.as_ref().display()),
                })
            }
        })
    }

    pub fn from_str(input: &str) -> Result<Self, ConfigError> {
        let s: Self = match serde_json::from_str(input) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::JsonParseError {
                    context: "Parse".into(),
                })
            }
        };

        Ok(s)
    }
}
