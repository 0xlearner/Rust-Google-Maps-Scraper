use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid JSON structure: {0}")]
    InvalidJson(String),

    #[error("No viewport found in the results")]
    NoViewport,

    #[error("Input is not an array")]
    NotAnArray,

    #[error("Failed to extract array from JSON structure")]
    ArrayExtractionFailed,

    #[error("Failed to extract value from JSON")]
    ValueExtractionFailed,
}
