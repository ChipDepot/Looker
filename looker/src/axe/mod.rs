mod contexter;

use axum::{routing::get, Router};
use log::info;
pub(crate) use contexter::get_location_context;

pub fn router() -> Router {
    Router::new().route(
        "/",
        get(|| async {
            info!("GET at /");
            "this is a message"
        }),
    )
}
