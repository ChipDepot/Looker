use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder};
use tokio::sync::Mutex;

use starduck::utils::get;
use starduck::utils::{
    CHANNEL, MAX_RETRY_INTERVAL, MIN_RETRY_INTERVAL, MQTT_URL, RETRY_CONNECTION_INTERVAL,
};

const DEFAULT_MQTT_URL: &str = "tcp://mqtt_broker:1883/";
const DEFAULT_CHANNEL: &str = "device-messages";

use crate::{app::traits::Processor, eyes::traits::Listener};

pub struct MQTTListener {
    pub connection: AsyncClient,
}

impl MQTTListener {
    pub async fn new() -> Self {
        let min_retry_interval = Duration::from_secs(get(MIN_RETRY_INTERVAL).unwrap_or(1));
        let max_retry_interval = Duration::from_secs(get(MAX_RETRY_INTERVAL).unwrap_or(16));
        let duration = get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
        let mqtt_url: String = get(MQTT_URL).unwrap_or(DEFAULT_MQTT_URL.to_owned());

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

#[async_trait::async_trait]
impl Listener for MQTTListener {
    async fn listen<T>(&mut self, obj: &mut Arc<Mutex<T>>) -> Result<()>
    where
        T: Processor + Send,
    {
        // let _duration = env_handler::get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
        let channel: String = get(CHANNEL).unwrap_or(DEFAULT_CHANNEL.to_owned());

        let reciever = self.connection.start_consuming();
        self.connection.subscribe(&channel, 1);

        loop {
            match reciever.recv() {
                Ok(Some(msg)) => obj.lock().await.process_message(msg),
                Ok(None) => {
                    warn!("Error when receiving message: got 'None' instead of Message");
                    continue;
                }
                Err(e) => {
                    warn!("Error when receiving message: {}", e);
                    continue;
                }
            };
        }

        // Ok(())
    }
}
