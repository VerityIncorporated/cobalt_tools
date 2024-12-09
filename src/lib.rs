pub mod api;
pub mod structs;

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{api::CobaltClient, structs::media_request::MediaRequestData};

    #[tokio::test]
    async fn test_status_success() {
        env::set_var("API_KEY", "dummy_api_key");
        env::set_var("INSTANCE_URI", "http://localhost:9000/");

        let client = CobaltClient.read().await;

        match client.status().await {
            Ok(response) => {
                println!("Response: {:#?}", response);
            }
            Err(e) => {
                panic!("Status request failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_video_success() {
        env::set_var("API_KEY", "your-api-key-here"); // Example: e81d0928-a69c-4b8e-8b6e-eab1d465ed31
        env::set_var("INSTANCE_URI", "http://localhost:9000");

        let client = CobaltClient.read().await;

        let video_data = MediaRequestData {
            url: "https://www.instagram.com/p/DCzseO1T9Xa".to_string(),
            video_quality: Some("1080".to_string()),
            audio_format: Some("mp3".to_string()),
            audio_bitrate: Some("320".to_string()),
            filename_style: Some("basic".to_string()),
            download_mode: Some("auto".to_string()),
            youtube_video_codec: Some("h264".to_string()),
            youtube_dub_lang: Some("en".to_string()),
            always_proxy: Some(false),
            disable_metadata: Some(false),
            tiktok_full_audio: Some(false),
            tiktok_h265: Some(false),
            twitter_gif: Some(true),
            youtube_hls: Some(false),
        };

        match client.get_media(None, video_data).await {
            Ok(response) => {
                println!("Response: {:#?}", response);
            }
            Err(media_error) => {
                eprintln!("Media Error: {:#?}", media_error);
            }
        }
    }
}