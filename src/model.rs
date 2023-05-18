use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct GetAgentResponse {
    pub data: AgentData,
}

#[derive(Debug, Deserialize)]
pub struct AgentData {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
}