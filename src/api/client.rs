use futures_util::StreamExt;
use once_cell::sync::Lazy;
use reqwest::{header::CONTENT_LENGTH, Client as ReqwestClient};
use std::{env, fs::File, io::Write, sync::Arc};
use tokio::sync::RwLock;

use crate::structs::{
    media_error::MediaError, media_request::MediaRequestData, media_response::Response,
    StatusResponse,
};

/// A client for interacting with the media service.
pub struct Client {
    api_key: String,
    instance_uri: String,
}

impl Client {
    /// Creates a new instance of the client.
    ///
    /// # Panics
    /// Panics if `API_KEY` or `INSTANCE_URI` are not set in the environment.
    pub(crate) fn new() -> Self {
        let api_key = env::var("API_KEY").expect("Expected API_KEY in the environment");
        let instance_uri =
            env::var("INSTANCE_URI").expect("Expected INSTANCE_URI in the environment");

        Client {
            api_key,
            instance_uri,
        }
    }

    /// Retrieves the status of the media service.
    ///
    /// # Returns
    /// - `Ok(StatusResponse)` containing the service status if the request succeeds.
    /// - `Err` if there is an error during the request or if the response cannot be parsed.
    ///
    /// # Example
    /// ```rust
    /// use cobalt_tools::api::CobaltClient;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Ensure API_KEY and INSTANCE_URI environment variables are set.
    ///     std::env::set_var("API_KEY", "your-api-key-here");
    ///     std::env::set_var("INSTANCE_URI", "http://localhost:9000");
    ///
    ///     let client = CobaltClient.read().await;
    ///
    ///     match client.status().await {
    ///         Ok(status) => println!("Service status: {:?}", status),
    ///         Err(err) => eprintln!("Error fetching status: {:?}", err),
    ///     }
    /// }
    /// ```
    pub async fn status(&self) -> Result<StatusResponse, Box<dyn std::error::Error + Send + Sync>> {
        let response = reqwest::get(self.instance_uri.clone())
            .await?
            .json::<StatusResponse>()
            .await?;

        Ok(response)
    }

    /// Retrieves the list of available services from the media service.
    ///
    /// This function makes a call to the `status` endpoint of the media service to
    /// obtain a list of active services. It extracts the `services` field from the
    /// response, which contains the names of the services currently available.
    ///
    /// # Errors
    /// This function will return an error in the following cases:
    /// - The `status` endpoint returns an error.
    /// - The `services` list in the `StatusResponse` is empty.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector containing the names of available services if the request succeeds.
    /// - `Err(Box<dyn std::error::Error + Send + Sync>)`: An error if the request fails or no services are found.
    ///
    /// # Examples
    /// ```rust
    /// use cobalt_tools::api::CobaltClient;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Set up the client instance
    ///     let client = CobaltClient.read().await;
    ///
    ///     // Fetch the list of services
    ///     match client.services().await {
    ///         Ok(services) => {
    ///             println!("Available services: {:?}", services);
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Failed to retrieve services: {}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn services(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let status = self.status().await?;

        if status.cobalt.services.is_empty() {
            return Err("No services found".into());
        }

        Ok(status.cobalt.services)
    }

    /// Fetches media based on the provided request data.
    ///
    /// # Parameters
    /// - `override_api_key`: An optional API key to override the default API key.
    /// - `video_data`: A `MediaRequestData` object containing the request payload.
    ///
    /// # Returns
    /// - `Ok(Response)` containing the fetched media response if the request succeeds.
    /// - `Err(MediaError)` if there is a request or deserialization error.
    ///
    /// # Example
    /// ```rust
    /// use cobalt_tools::api::CobaltClient;
    /// use cobalt_tools::structs::media_request::MediaRequestData;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     // Ensure API_KEY and INSTANCE_URI environment variables are set.
    ///     std::env::set_var("API_KEY", "your-api-key-here");
    ///     std::env::set_var("INSTANCE_URI", "http://localhost:9000");
    ///
    ///     let client = CobaltClient.read().await;
    ///
    ///     let video_data = MediaRequestData {
    ///         url: "https://www.youtube.com/watch?v=1lML-Uem6Ns".to_string(),
    ///         filename_style: "basic".to_string(),
    ///         ..Default::default()
    ///     };
    ///
    ///     match client.get_media(None, video_data).await {
    ///         Ok(response) => {
    ///             println!("Response: {:#?}", response);
    ///         }
    ///         Err(media_error) => {
    ///             eprintln!("Media Error: {:#?}", media_error);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_media(
        &self,
        override_api_key: Option<String>,
        video_data: MediaRequestData,
    ) -> Result<Response, MediaError> {
        let api_key = override_api_key.unwrap_or(self.api_key.clone());

        let serialized = serde_json::to_string(&video_data).unwrap();

        let client = ReqwestClient::new();
        let response = client
            .post(self.instance_uri.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "Cobalt")
            .header("Authorization", format!("Api-Key {}", api_key))
            .body(serialized)
            .send()
            .await;

        let response = match response {
            Ok(r) => r,
            Err(e) => {
                return Err(MediaError::RequestError(format!(
                    "Failed to send request: {}",
                    e
                )))
            }
        };

        if !response.status().is_success() {
            return Err(MediaError::ApiError(format!(
                "API request failed with status: {} | {:?}",
                response.status(),
                response.text().await
            )));
        }

        let final_response: Response = match response.json().await {
            Ok(res) => res,
            Err(e) => {
                return Err(MediaError::DeserializationError(format!(
                    "Failed to parse response: {}",
                    e
                )))
            }
        };

        Ok(final_response)
    }

    pub async fn download(
        tunnel_link: String,
        path: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(&tunnel_link).await?;
    
        if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
            let content_length = content_length.to_str()?.parse::<u64>()?;
    
            if content_length == 0 {
                eprintln!("The file has a content length of 0 bytes. Something went wrong.");
                return Err("File has a content length of 0 bytes.".into());
            }
        } else {
            eprintln!("Content-Length header is missing. Something went wrong.");
            return Err("Content-Length header is missing.".into());
        }
    
        if !response.status().is_success() {
            eprintln!("Failed to download file: HTTP {}", response.status());
            return Err(format!("Failed to download file: HTTP {}", response.status()).into());
        }
    
        let mut file = File::create(path).expect("Failed to create file");
    
        let mut content = response.bytes_stream();
        while let Some(chunk) = content.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
        }
    
        Ok(())
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

pub static CLIENT_INSTANCE: Lazy<Arc<RwLock<Client>>> =
    Lazy::new(|| Arc::new(RwLock::new(Client::new())));
