use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;

pub enum ConfigError {
    NotFound { name: String },
    InternalServerError { context: String },
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    from_file: Option<Vec<String>>,
    pub template_name: String,
    pub context: JsonValue,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        Self::from_str(match &std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                return Err(ConfigError::NotFound {
                    name: format!("Path: {}", path),
                })
            }
        })
    }

    pub fn from_str(input: &str) -> Result<Self, ConfigError> {
        let mut s: Self = match serde_json::from_str(input) {
            Ok(c) => c,
            Err(_) => {
                return Err(ConfigError::InternalServerError {
                    context: "Parse".into(),
                })
            }
        };

        if let Some(fields) = &s.from_file {
            for field_name in fields {
                let field = match s.context.pointer_mut(field_name) {
                    Some(f) => f,
                    None => {
                        return Err(ConfigError::InternalServerError {
                            context: "Missing Field".into(),
                        })
                    }
                };

                let filename = field.as_str();

                if let Some(filename) = filename {
                    *field = json!(match std::fs::read_to_string(filename) {
                        Ok(s) => s,
                        Err(_) =>
                            return Err(ConfigError::InternalServerError {
                                context: "Replace Content".into()
                            }),
                    });
                } else {
                    return Err(ConfigError::InternalServerError {
                        context: "Type".into(),
                    });
                }
            }
        }

        Ok(s)
    }
}
