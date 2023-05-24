mod config;
mod models;
mod service;
use service::core::get_agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let config = config::read_config("config.toml")?;
    // println!("Parsed config: {:?}", config);

    let result = get_agent(config.api.url, config.api.api_key).await;
    match result {
        Ok(data) => println!("Result: {:?}", data),
        Err(e) if e.is_decode() => {
            println!("Failed to deserialze the response: {}", e);
        }
        Err(e) => {
            println!("Request failed: {}", e);
        }
    }

    Ok(())
}

