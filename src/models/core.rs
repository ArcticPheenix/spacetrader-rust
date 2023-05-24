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

#[derive(Debug, Deserialize)]
pub struct PostRegister {
    pub data: PubRegisterData,
}

#[derive(Debug, Deserialize)]
pub struct PubRegisterData {
    pub symbol: String,
    pub faction: String,
}
