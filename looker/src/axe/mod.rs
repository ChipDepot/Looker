mod contexter;
mod requester;

pub(crate) use contexter::get_location_context;
pub(crate) use requester::send_context;

use std::path::PathBuf;

use axum::{routing::get, Router};
use tower_http::services::ServeFile;

use requester::get_application;

pub fn router() -> Router {
    Router::new().route("/app", get(get_application))
    // .route("/", get(message))
}

pub(crate) fn extras_router() -> Router {
    Router::new().route_service(
        "/favicon.ico",
        ServeFile::new(PathBuf::from("assets/favicon.ico")),
    )
}
