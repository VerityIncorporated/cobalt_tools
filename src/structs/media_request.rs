use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MediaRequestData<'a> {
    pub url: &'a str,
    #[serde(rename = "videoQuality", skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<&'a str>,
    #[serde(rename = "audioFormat", skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<&'a str>,
    #[serde(rename = "audioBitrate", skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<&'a str>,
    #[serde(rename = "filenameStyle")]
    pub filename_style: &'a str,
    #[serde(rename = "downloadMode", skip_serializing_if = "Option::is_none")]
    pub download_mode: Option<&'a str>,
    #[serde(rename = "youtubeVideoCodec", skip_serializing_if = "Option::is_none")]
    pub youtube_video_codec: Option<&'a str>,
    #[serde(rename = "youtubeDubLang", skip_serializing_if = "Option::is_none")]
    pub youtube_dub_lang: Option<&'a str>,
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