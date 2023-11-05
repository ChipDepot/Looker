mod app;
mod axe;
mod red;
mod utils;

#[macro_use]
extern crate log;

use app::traits::Listener;
use paho_mqtt::message;
use red::redis::RedisListener;
use utils::{env_handler, env_keys::PORT};

use axum::{Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Start the logger and load the env variables
    env_logger::init();
    env_handler::load_env(None);

    let mut application = axe::get_location_context().await.unwrap();

    tokio::spawn(async move { RedisListener::new().listen(&mut application) });

    let app = Router::new().merge(axe::router());

    let addr = SocketAddr::from(([0, 0, 0, 0], env_handler::get(PORT).unwrap()));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
