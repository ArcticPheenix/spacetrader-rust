mod config;
mod model;
use model::GetAgentResponse;
use reqwest::{ Client };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let config = config::read_config("config.toml")?;
    println!("Parsed config: {:?}", config);

    let result = get_agent(config.api.url, config.api.api_key).await?;
    println!("Result: {:?}", result);
    
    Ok(())
}

async fn get_agent(url: String, api_key: String) -> Result<GetAgentResponse, reqwest::Error>{
    let agent_url = url + "/v2/my/agent";
    let client = Client::new();
    let response = client.get(agent_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .json::<model::GetAgentResponse>()
        .await?;
    Ok(response)
}