use crate::models::core::GetAgentResponse;
use reqwest::Client;
pub async fn get_agent(url: String, api_key: String) -> Result<GetAgentResponse, reqwest::Error> {
    let agent_url = url + "/v2/my/agent";
    let client = Client::new();
    let response = client
        .get(agent_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?
        .error_for_status()?
        .json::<GetAgentResponse>()
        .await?;
    Ok(response)
}
