mod app;
mod axe;
mod eyes;
mod utils;

#[macro_use]
extern crate log;
use eyes::mqtt::MQTTListener;
use eyes::traits::Listener;
use utils::{env_handler, env_keys::PORT};

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Start the logger and load the env variables
    env_logger::init();
    env_handler::load_env(None);

    let mut application = axe::get_location_context().await.unwrap();

    tokio::spawn(async move {
        let _ = MQTTListener::new().await.listen(&mut application);
    });

    let app = Router::new().merge(axe::router());
    let addr = SocketAddr::from(([0, 0, 0, 0], env_handler::get(PORT).unwrap()));
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();
}
