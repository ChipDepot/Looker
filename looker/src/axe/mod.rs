use axum::{routing::get, Router};
use log::info;

pub fn router() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            info!("GET at /");
            "this is a message"
        }),
    )
}
