use axum::{http::Response, Json};
use reqwest::{Client, StatusCode};
use starduck::application;
use std::env;

use crate::env_handler;
use crate::utils::env_keys::{APP_NAME, BRAN_IP};

async fn get_location_context() -> Result<Json<application::Application>, reqwest::Error> {
    let bran_endpoint = format!(
        "{}/{}",
        env_handler::get::<String>(BRAN_IP).unwrap(),
        env_handler::get::<String>(APP_NAME).unwrap()
    );
    let client = Client::new();

    loop {
        let response = client.get(&bran_endpoint).send().await?;

        if response.status() != StatusCode::OK {
            warn!(
                "Waiting for Location Context for app {} from {}",
                env_handler::get::<String>(APP_NAME).unwrap(),
                &bran_endpoint
            );
        }

        // let response_bytes = response.bytes().await?;
        response.json().await?;
    }
}
