mod app;
mod axe;
mod eyes;
mod utils;

#[macro_use]
extern crate log;

use std::{net::SocketAddr, process::exit, sync::Arc};

use axum::{Extension, Router};
use tokio::{net::TcpListener, sync::Mutex};

use crate::{
    eyes::MQTTListener,
    eyes::{clock, Listener},
    utils::PORT,
    utils::{get, load_env},
};

#[tokio::main]
async fn main() {
    // Start the logger and load the env variables
    env_logger::init();
    load_env(None);

    // Create the application and Arc<Mutex<T>> it
    let application = axe::get_location_context().await.unwrap();
    let mut arc_app = Arc::new(Mutex::new(application));
    let mut arc_clone = Arc::clone(&arc_app);
    let arc_axum_clone = Arc::clone(&arc_app);

    let clock_task = tokio::spawn(async move { clock(&mut arc_app).await });

    let mqtt_listener_task =
        tokio::spawn(async move { MQTTListener::new().await.listen(&mut arc_clone).await });
    // listener.

    let app = Router::new()
        .nest_service("/", axe::extras_router())
        .nest("/", axe::router())
        .layer(Extension(arc_axum_clone));
    let addr = SocketAddr::from(([0, 0, 0, 0], get(PORT).unwrap()));
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    info!("Starting server at {}", addr);

    let _axum_server = axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await;

    let _ = tokio::select! {
        _ = clock_task =>{
            error!("Clock task finished unexpectedly");
            exit(-1);
        },
        _ = mqtt_listener_task =>{
            error!("MQTTListener task finished unexpectedly");
            exit(-1);
        },
    };
}
