mod red;
mod utils;

#[macro_use]
extern crate log;

use red::connector::RedisConnector;
use utils::{env_handler, env_keys};

fn main() {
    // Set log level to debug so all things show up and start the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Load the env variables
    env_handler::load_env(None);

}
