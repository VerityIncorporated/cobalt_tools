use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub cobalt: Cobalt,
    pub git: Git,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cobalt {
    pub version: String,
    pub url: String,
    pub start_time: String,
    pub duration_limit: u64,
    pub services: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Git {
    pub branch: String,
    pub commit: String,
    pub remote: String,
}