extern crate redis;

use crate::utils::env_keys::{REDIS_CHANNEL, REDIS_URL};
use log::info;
use std::env;

pub struct RedisListener {
    connection: redis::Connection,
}

impl RedisListener {
    pub fn new() -> Self {
        // Get the redis URL from the env vars
        let redis_url = dotenv::var(REDIS_URL).unwrap_or_else(|err| panic!("Missing {}", err));
        info!("Using redis url {}", &redis_url);

        let redis_client = redis::Client::open(redis_url).unwrap_or_else(|err| panic!("{}", err));

        info!("Establising redis connection");
        let conn = redis_client
            .get_connection()
            .unwrap_or_else(|err| panic!("{}", err));
        info!("Created redis connector");

        return RedisListener { connection: conn };
    }

    pub(crate) fn listen(&mut self, f: fn(String)) -> redis::RedisResult<()> {
        // let mut counter: usize = 0;
        let queue_channel = env::var(REDIS_CHANNEL)
            .unwrap_or_else(|err| panic!("Missing env var {}: {}", REDIS_CHANNEL, err));

        let mut pubsub_con = self.connection.as_pubsub();
        pubsub_con.subscribe(&queue_channel)?;

        loop {
            let message = pubsub_con.get_message().and_then(|msg| {
                let bytes = msg.get_payload_bytes();
                Ok(String::from_utf8_lossy(bytes).to_string())
            })?;

            info!("New message from redis: {}", &message);
            f(message);

            // counter += 1;

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
