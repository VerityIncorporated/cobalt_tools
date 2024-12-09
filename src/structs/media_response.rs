use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Status {
    Error,
    Picker,
    Redirect
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorContext {
    pub service: Option<String>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub context: Option<ErrorContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaItem {
    pub r#type: String,
    pub url: String,
    pub thumb: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PickerResponse {
    pub status: String,
    pub audio: Option<String>,
    pub audio_filename: Option<String>,
    pub picker: Vec<MediaItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectResponse {
    pub status: String,
    pub url: String,
    pub filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Error(ErrorResponse),
    Picker(PickerResponse),
    Redirect(RedirectResponse)
}

impl Response {
    pub fn get_status(&self) -> Status {
        match self {
            Response::Error(_) => Status::Error,
            Response::Picker(_) => Status::Picker,
            Response::Redirect(_) => Status::Redirect,
        }
    }
}