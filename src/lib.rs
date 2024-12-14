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
            url: "https://www.youtube.com/watch?v=1lML-Uem6Ns".to_string(),
            filename_style: "basic".to_string(),
            ..Default::default()
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