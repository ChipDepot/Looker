extern crate redis;

use std::{sync::Arc, thread, time};

use anyhow::Result;
use log::info;
use tokio::sync::Mutex;

use starduck::utils::get;
use starduck::utils::{CHANNEL, REDIS_URL, RETRY_CONNECTION_INTERVAL};

use crate::{app::traits::Processor, eyes::traits::Listener};

pub struct RedisListener {
    connection: redis::Connection,
}

impl RedisListener {
    pub fn new() -> Self {
        let duration = get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);

        // Get the redis URL from the env vars
        let redis_url = dotenv::var(REDIS_URL).unwrap_or_else(|err| panic!("Missing {}", err));
        info!("Using redis url {}", &redis_url);

        let redis_client = redis::Client::open(redis_url).unwrap_or_else(|err| panic!("{}", err));

        info!("Establising redis connection...");

        loop {
            match redis_client.get_connection() {
                Ok(k) => {
                    info!("Created redis connector");
                    return RedisListener { connection: k };
                }
                Err(e) => {
                    warn!("Unable to stablish redis connection: {}", e.to_string());
                    warn!("Retrying in {} seconds", &duration);
                    thread::sleep(time::Duration::from_secs(duration));
                }
            };
        }
    }
}

#[async_trait::async_trait]
impl Listener for RedisListener {
    async fn listen<T: Processor>(&mut self, _obj: &mut Arc<Mutex<T>>) -> Result<()> {
        let duration = get::<u64>(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
        let queue_channel = get::<String>(CHANNEL)
            .unwrap_or_else(|err| panic!("Missing env var {}: {}", CHANNEL, err));

        let mut pubsub_con = self.connection.as_pubsub();

        loop {
            match pubsub_con.subscribe(&queue_channel) {
                Ok(_) => break,
                Err(e) => warn!(
                    "Unable to stablish subscribe to channel {}: {}",
                    &queue_channel,
                    e.to_string()
                ),
            };
            warn!("Retrying in {} seconds", &duration);
            thread::sleep(time::Duration::from_secs(duration));
        }

        loop {
            let message = pubsub_con.get_message().and_then(|msg| {
                let bytes = msg.get_payload_bytes();
                Ok(String::from_utf8_lossy(bytes).to_string())
            })?;

            info!("New message from redis: {}", &message);
            // obj.process_message(&message);
        }
    }
}
