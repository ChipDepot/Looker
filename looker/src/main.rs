mod axe;
mod red;
mod utils;

#[macro_use]
extern crate log;

use red::listener::RedisListener;
use utils::{env_handler, env_keys::PORT};

use axum::{Router, Server};
use std::net::SocketAddr;
use tokio::select;

#[tokio::main]
async fn main() {
    // Start the logger and load the env variables
    env_logger::init();
    // tracing_subscriber::fmt().json().init();
    env_handler::load_env(None);

    tokio::spawn(async { RedisListener::new().listen() });

    let app = Router::new().merge(axe::router());

    let addr = SocketAddr::from(([0, 0, 0, 0], env_handler::get(PORT).unwrap()));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
