use config::create_config;
use config::Config;
use simple_logger::SimpleLogger;
use log::error;

fn main() {
    SimpleLogger::new().init().unwrap();
    let configuration: Config;
    match create_config() {
        Some(config) => configuration = config,
        None => {
            error!("couldn't read configuration");
            return
        }
    }

    println!(
        "Key: {}; Secret: {}", 
        configuration.blizzard_client_key, 
        configuration.blizzard_client_secret
    )
}