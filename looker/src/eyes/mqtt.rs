use std::thread;
use std::time::Duration;

use paho_mqtt::{errors::Error, AsyncClient, ConnectOptionsBuilder};

use crate::{
    app::traits::Processor,
    eyes::traits::Listener,
    utils::{
        env_handler,
        env_keys::{
            CHANNEL, MAX_RETRY_INTERVAL, MIN_RETRY_INTERVAL, MQTT_URL, RETRY_CONNECTION_INTERVAL,
        },
    },
};

pub struct MQTTListener {
    pub connection: AsyncClient,
}

impl MQTTListener {
    pub async fn new() -> Self {
        let min_retry_interval = Duration::from_secs(
            env_handler::get(MIN_RETRY_INTERVAL)
                .expect("Missing keyword MIN_RETRY_INTERVAL on .env file"),
        );
        let max_retry_interval = Duration::from_secs(
            env_handler::get(MAX_RETRY_INTERVAL)
                .expect("Missing keyword MIN_RETRY_INTERVAL on .env file"),
        );
        let duration = env_handler::get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
        let mqtt_url: String = env_handler::get(MQTT_URL).unwrap();

        info!("Using URL {}", &mqtt_url);
        let mqtt_client = AsyncClient::new(mqtt_url).unwrap();

        let conn_opts = ConnectOptionsBuilder::new()
            .automatic_reconnect(min_retry_interval, max_retry_interval)
            .clean_session(true)
            .finalize();

        // Connect and wait for it to complete or fail
        info!("Establising connection...");

        loop {
            mqtt_client.connect(conn_opts.clone()).await.unwrap();

            if mqtt_client.is_connected() {
                info!("Created MQTT connection");
                return MQTTListener {
                    connection: mqtt_client,
                };
            } else {
                warn!("Unable to stablish MQTT connection");
                warn!("Retrying in {} seconds", &duration);
                thread::sleep(Duration::from_secs(duration));
            };
        }
    }
}

impl Listener for MQTTListener {
    type K = Error;

    fn listen<T: Processor>(&mut self, obj: &mut T) -> Result<(), Self::K> {
        let _duration = env_handler::get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
        let channel: String = env_handler::get(CHANNEL).unwrap();

        let reciever = self.connection.start_consuming();
        self.connection.subscribe(&channel, 1);

        loop {
            match reciever.recv() {
                Ok(Some(msg)) => obj.process_message(msg),
                Ok(None) => {
                    warn!("Error when receiving message: got 'None' instead of Message");
                    continue;
                }
                Err(e) => {
                    warn!("Error when receiving message: {}", e);
                    continue;
                }
            };

            // info!("message received from topic: `{}`", received.topic());
            // info!("message: `{}`", received.payload_str());
        }
    }
}
