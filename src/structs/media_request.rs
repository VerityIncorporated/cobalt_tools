use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MediaRequestData {
    pub url: String,
    #[serde(rename = "videoQuality", skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
    #[serde(rename = "audioFormat", skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<String>,
    #[serde(rename = "audioBitrate", skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<String>,
    #[serde(rename = "filenameStyle")]
    pub filename_style: String,
    #[serde(rename = "downloadMode", skip_serializing_if = "Option::is_none")]
    pub download_mode: Option<String>,
    #[serde(rename = "youtubeVideoCodec", skip_serializing_if = "Option::is_none")]
    pub youtube_video_codec: Option<String>,
    #[serde(rename = "youtubeDubLang", skip_serializing_if = "Option::is_none")]
    pub youtube_dub_lang: Option<String>,
    #[serde(rename = "alwaysProxy", skip_serializing_if = "Option::is_none")]
    pub always_proxy: Option<bool>,
    #[serde(rename = "disableMetadata", skip_serializing_if = "Option::is_none")]
    pub disable_metadata: Option<bool>,
    #[serde(rename = "tiktokFullAudio", skip_serializing_if = "Option::is_none")]
    pub tiktok_full_audio: Option<bool>,
    #[serde(rename = "tiktokH265", skip_serializing_if = "Option::is_none")]
    pub tiktok_h265: Option<bool>,
    #[serde(rename = "twitterGif", skip_serializing_if = "Option::is_none")]
    pub twitter_gif: Option<bool>,
    #[serde(rename = "youtubeHLS", skip_serializing_if = "Option::is_none")]
    pub youtube_hls: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub enum DownloadMode {
    Auto,
    Audio,
    Mute,
}

impl DownloadMode {
    pub fn to_string(&self) -> &str {
        match self {
            DownloadMode::Auto => "auto",
            DownloadMode::Audio => "audio",
            DownloadMode::Mute => "mute",
        }
    }

    pub fn from_str(input: &str) -> Option<DownloadMode> {
        match input {
            "auto" => Some(DownloadMode::Auto),
            "audio" => Some(DownloadMode::Audio),
            "mute" => Some(DownloadMode::Mute),
            _ => None,
        }
    }
}