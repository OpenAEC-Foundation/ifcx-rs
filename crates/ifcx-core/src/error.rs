//! Error types for ifcx-core.

#[derive(Debug, thiserror::Error)]
pub enum IfcxError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Missing required entry: {0}")]
    MissingEntry(String),

    #[error("Missing required attribute '{attribute}' on entry '{path}'")]
    MissingAttribute { path: String, attribute: String },

    #[error("Invalid IFC class: expected {expected}, got {got}")]
    InvalidClass { expected: String, got: String },

    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, IfcxError>;
