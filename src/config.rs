use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub url: String,
    pub api_key: String,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}