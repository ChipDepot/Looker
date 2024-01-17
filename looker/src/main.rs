mod app;
mod axe;
mod eyes;

#[macro_use]
extern crate log;

use std::{net::SocketAddr, process::exit, sync::Arc};

use axum::{Extension, Router};
use tokio::{net::TcpListener, sync::Mutex};

use starduck::utils;
use starduck::utils::PORT;

const DEFAULT_PORT: u16 = 3000;

use crate::{
    eyes::MQTTListener,
    eyes::{clock, Listener},
};

#[tokio::main]
async fn main() {
    // Start the logger
    env_logger::init();

    // Create the application and Arc<Mutex<T>> it
    let application = axe::get_location_context().await.unwrap();
    let mut clock_arc = Arc::new(Mutex::new(application));
    let mut listener_arc = Arc::clone(&clock_arc);
    let requester_arc = Arc::clone(&clock_arc);
    let axum_arc = Arc::clone(&clock_arc);

    let clock_task = tokio::spawn(async move { clock(&mut clock_arc).await });

    let mqtt_listener_task =
        tokio::spawn(async move { MQTTListener::new().await.listen(&mut listener_arc).await });

    let requester_task = tokio::spawn(async move { axe::send_context(requester_arc).await });

    let app = Router::new()
        .nest_service("/", axe::extras_router())
        .nest("/", axe::router())
        .layer(Extension(axum_arc));
    let addr = SocketAddr::from(([0, 0, 0, 0], utils::get(PORT).unwrap_or(DEFAULT_PORT)));
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    info!("Starting server at {}", addr);

    let _axum_server = axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await;

    let _ = tokio::select! {
        _ = clock_task => {
            error!("Clock task finished unexpectedly");
            exit(-1);
        },
        _ = mqtt_listener_task => {
            error!("MQTTListener task finished unexpectedly");
            exit(-1);
        },
        _ = requester_task => {
            error!("MQTTListener task finished unexpectedly");
            exit(-1);
        }
    };
}
