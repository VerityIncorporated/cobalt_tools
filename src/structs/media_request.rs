use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MediaRequestData {
    pub url: String,
    #[serde(rename = "videoQuality")]
    pub video_quality: Option<String>,
    #[serde(rename = "audioFormat")]
    pub audio_format: Option<String>,
    #[serde(rename = "audioBitrate")]
    pub audio_bitrate: Option<String>,
    #[serde(rename = "filenameStyle")]
    pub filename_style: Option<String>,
    #[serde(rename = "downloadMode")]
    pub download_mode: Option<String>,
    #[serde(rename = "youtubeVideoCodec")]
    pub youtube_video_codec: Option<String>,
    #[serde(rename = "youtubeDubLang")]
    pub youtube_dub_lang: Option<String>,
    #[serde(rename = "alwaysProxy")]
    pub always_proxy: Option<bool>,
    #[serde(rename = "disableMetadata")]
    pub disable_metadata: Option<bool>,
    #[serde(rename = "tiktokFullAudio")]
    pub tiktok_full_audio: Option<bool>,
    #[serde(rename = "tiktokH265")]
    pub tiktok_h265: Option<bool>,
    #[serde(rename = "twitterGif")]
    pub twitter_gif: Option<bool>,
    #[serde(rename = "youtubeHLS")]
    pub youtube_hls: Option<bool>
}