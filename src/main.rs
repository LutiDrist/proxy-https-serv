use proxy_https_serv::{config, logging, server, metrics::{REQUEST_COUNT, REQUEST_DURATION}};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging();
    info!("Starting proxy-https-serv");

    let config = config::load_config("config.toml")?;
    info!("Configuration loaded: port={}", config.port);

    info!("Metrics initialized: request_count={}, request_duration_sample_count={}",REQUEST_COUNT.get(), REQUEST_DURATION.get_sample_count());

    server::start_server("config.toml", &config.cert_path, &config.key_path).await?;

    Ok(())
}