use std::fmt;

#[derive(Debug)]
pub enum MediaError {
    RequestError(String),
    DeserializationError(String),
    ApiError(String)
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaError::RequestError(msg) => write!(f, "Request Error: {}", msg),
            MediaError::DeserializationError(msg) => write!(f, "Deserialization Error: {}", msg),
            MediaError::ApiError(msg) => write!(f, "API Error: {}", msg),
        }
    }
}