use anyhow::Result;
use reqwest::{Client, StatusCode};
use starduck::Application;
use url::Url;

use starduck::utils::get;
use starduck::utils::{APP_NAME, BRAN_URL, RETRY_CONNECTION_INTERVAL};

pub async fn get_location_context() -> Result<Application, reqwest::Error> {
    let duration = get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
    let app_name = get::<String>(APP_NAME).unwrap();
    let bran_endpoint = match get::<Url>(BRAN_URL) {
        Ok(url) => url.join(&format!("apps/{app_name}")).unwrap(),
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
                        get::<String>(APP_NAME).unwrap(),
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
