use reqwest::{Client, StatusCode};
use starduck::application::Application;
use url::Url;

use crate::env_handler;
use crate::utils::env_keys::{APP_NAME, BRAN_URL, RETRY_CONNECTION_INTERVAL};

pub(crate) async fn get_location_context() -> Result<Application, reqwest::Error> {
    let duration = env_handler::get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
    let app_name = env_handler::get::<String>(APP_NAME).unwrap();
    let bran_endpoint = match env_handler::get::<Url>(BRAN_URL) {
        Ok(url) => url.join(&app_name).unwrap(),
        Err(err) => panic!("Failed to get bran URL: {}", err),
    };

    let client = Client::new();

    loop {
        let response = match client.get(bran_endpoint.as_str()).send().await {
            Ok(r) => Some(r),
            Err(_) => None,
        };

        match response {
            Some(k) => match k.status() {
                StatusCode::OK => {
                    return k.json().await;
                }
                StatusCode::NOT_FOUND => {
                    info!(
                        "Waiting for Location Context for app {} from {}",
                        env_handler::get::<String>(APP_NAME).unwrap(),
                        &bran_endpoint
                    );
                }
                StatusCode::SERVICE_UNAVAILABLE => {
                    warn!(
                        "Couldn't access {}. Is the service running?",
                        &bran_endpoint
                    );
                }
                _ => {
                    warn!(
                        "Unexpected HTTP response {} from {}",
                        k.status(),
                        &bran_endpoint
                    );
                }
            },
            None => warn!(
                "Couldn't access {}. Is the service running?",
                &bran_endpoint
            ),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;
    }
}
