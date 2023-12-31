use std::{net::SocketAddr, sync::Arc};

use tokio::sync::Mutex;

use axum::{
    extract::{ConnectInfo, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};

use starduck::Application;
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
