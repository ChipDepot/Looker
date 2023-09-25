extern crate redis;
use redis::{Commands, ConnectionLike};

use crate::utils::env_keys::REDIS_URL;
use log::{info, warn};

pub(crate) struct RedisListener {
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

    pub(crate) fn listen(&mut self) -> redis::RedisResult<()> {
        // let mut counter: usize = 0;

        self.connection.set(String::from("counter"), 0)?;

        loop {
            match self.connection.get("counter") {
                Ok(response) => match response {
                    redis::Value::Status(value) => info!("Response from redis: {}", value),
                    redis::Value::Int(value) => info!("Response from redis: {}", value),
                    redis::Value::Data(value) => {
                        info!("Response from redis: {}", String::from_utf8_lossy(&value));
                        self.connection.incr("counter", 1)?;
                    }
                    _ => warn!("Odd response from redis: {:?}", response),
                },
                Err(err) => warn!("Error from Redis: {}", err),
            };

            // counter += 1;
            // info!("We're at {}", counter);

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
