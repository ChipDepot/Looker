extern crate redis;

use crate::{
    app::traits::Processor,
    eyes::traits::Listener,
    utils::{
        env_handler,
        env_keys::{CHANNEL, REDIS_URL, RETRY_CONNECTION_INTERVAL},
    },
};
use log::info;
use redis::RedisError;
use std::{thread, time};

pub struct RedisListener {
    connection: redis::Connection,
}

impl RedisListener {
    pub fn new() -> Self {
        let duration = env_handler::get(RETRY_CONNECTION_INTERVAL).unwrap_or(10);

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

impl Listener for RedisListener {
    type K = RedisError;

    fn listen<T: Processor>(&mut self, _obj: &mut T) -> Result<(), Self::K> {
        let duration = env_handler::get::<u64>(RETRY_CONNECTION_INTERVAL).unwrap_or(10);
        let queue_channel = env_handler::get::<String>(CHANNEL)
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
