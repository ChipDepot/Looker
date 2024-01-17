use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use reqwest::StatusCode as ReqStatusCode;
use starduck::Application;
use tokio::sync::Mutex;
use url::Url;

use starduck::utils::get;
use starduck::utils::{BRAN_URL, FORWARD_INTERVAL};

const BRAN_DEFAULT: &str = "http://bran:8014";
const DEFAULT_FORWARD_INTERVAL: u64 = 120;

use axum::{
    extract::{ConnectInfo, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

pub async fn get_application(
    Extension(app): Extension<Arc<Mutex<Application>>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    // Path(app_name): Path<String>,
) -> Response {
    let app_clone = app.lock().await.clone();

    let app_name = app_clone.name.clone();
    info!("Get for {} request from {}", app_name, addr);

    let json_response = Json(app_clone);

    info!("{} info sent to {}", app_name, addr);
    return (StatusCode::OK, json_response).into_response();
}

pub async fn send_context(app_arc: Arc<Mutex<Application>>) -> Result<()> {
    let forward_interval = get::<u64>(FORWARD_INTERVAL).unwrap_or(DEFAULT_FORWARD_INTERVAL);
    let sleep_interval = Duration::from_secs(forward_interval);

    let app_name = app_arc.lock().await.name.clone();

    let bran_endpoint = match get::<Url>(BRAN_URL) {
        Ok(url) => url.join(&format!("/apps/{app_name}")).unwrap(),
        Err(_) => Url::from_str(&format!("{BRAN_DEFAULT}/apps/{app_name}")).unwrap(),
    };

    loop {
        sleep(sleep_interval);

        let app = app_arc.lock().await.clone();

        let client = Client::new();

        if let Ok(k) = client.put(bran_endpoint.clone()).json(&app).send().await {
            if k.status() != ReqStatusCode::OK {
                error!("Recived {} from {}", k.status(), bran_endpoint.to_string());
                continue;
            }
        };

        info!("Updated application {app_name} has been POSTed to {bran_endpoint}");
    }
}
