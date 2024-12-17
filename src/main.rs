mod config;
mod server;

use crate::config::Config;
use crate::server::start_server;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt().init();

    // Load configuration
    let config = Config::load("config/default.toml").expect("Failed to load configuration");

    info!("Starting server on {}:{}", config.address, config.port);

    // Start the server
    start_server(config).await;
}
