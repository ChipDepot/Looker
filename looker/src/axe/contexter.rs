use axum::{http::Response, Json};
use reqwest::{Client, StatusCode};
use starduck::location::Location;
use std::env;

use crate::env_handler;
use crate::utils::env_keys::{APP_NAME, BRAN_IP, CONTEXT_INTERVAL};

pub(crate) async fn get_location_context() -> Result<Location, reqwest::Error> {
    let duration = env_handler::get(CONTEXT_INTERVAL).unwrap_or(10);
    let bran_endpoint = format!(
        "{}/{}",
        env_handler::get::<String>(BRAN_IP).unwrap(),
        env_handler::get::<String>(APP_NAME).unwrap()
    );
    let client = Client::new();

    loop {
        let response = client.get(&bran_endpoint).send().await?;

        match response.status() {
            StatusCode::OK => {
                return response.json().await;
            },
            StatusCode::NOT_FOUND => {
                info!(
                    "Waiting for Location Context for app {} from {}",
                    env_handler::get::<String>(APP_NAME).unwrap(),
                    &bran_endpoint
                );
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                warn!(
                    "Couldn't access {}. Is the service running?",
                    &bran_endpoint
                );
            },
            _ => {
                warn!("Unexpected HTTP response {} from {}", response.status(), &bran_endpoint);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(duration)).await;

    }
}
