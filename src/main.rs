mod config;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let config = config::read_config("config.toml")?;
    println!("Parsed config: {:?}", config);

    Ok(())
}
