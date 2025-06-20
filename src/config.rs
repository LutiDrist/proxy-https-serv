use serde::Deserialize;
use std::fs;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub backends: Vec<String> ,
    pub cert_path: String,
    pub key_path: String,
}


pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}