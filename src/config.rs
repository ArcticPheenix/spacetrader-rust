use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    api: ApiConfig,
}

#[derive(Debug, Deserialize)]
struct ApiConfig {
    url: String,
    api_key: String,
}