extern crate dotenv;
extern crate log;

use dotenv::dotenv;
use std::env;
use log::error;


pub struct Config {
    pub blizzard_client_key: String,
    pub blizzard_client_secret: String,
}

pub fn create_config() -> Option<Config> {
    dotenv().ok();

    let blizzard_client_key: String;
    let blizzard_client_secret: String;

    match env::var("BLIZZARD_CLIENT_KEY") {
        Ok(val) => blizzard_client_key = val,
        _ => {
            error!("No BLIZZARD_CLIENT_KEY in config");
            return None
        }
    }

    match env::var("BLIZZARD_CLIENT_SECRET") {
        Ok(val) => blizzard_client_secret = val,
        _ => {
            error!("No BLIZZARD_CLIENT_SECRET in config");
            return None
        }
    }

    Some(Config {
        blizzard_client_key: blizzard_client_key,
        blizzard_client_secret: blizzard_client_secret
    })
}
