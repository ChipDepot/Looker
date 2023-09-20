use std::path::PathBuf;
use crate::utils::env_keys::REDIS_URL;
use dotenv;

use redis;

pub struct RedisConnector {
    pub connection: redis::Connection
}

impl RedisConnector {
    pub fn new(self) -> Self {
        // Get the redis URL from the env vars
        let redis_url = dotenv::var(REDIS_URL)
            .unwrap_or_else(|err| panic!("{}", err));

        let conn = redis::Client::open(redis_url)
            .unwrap_or_else(|err| panic!("{}", err));

        todo!();
    }


    fn get_settings(self, path: Option<PathBuf>) {
        todo!()
    }
}