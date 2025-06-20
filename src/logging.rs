use log::info;
use env_logger;

pub fn init_logging() {
    env_logger::init();
    info!("initializing logging");
}