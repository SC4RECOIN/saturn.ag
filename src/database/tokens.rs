use anyhow::Result;
use postgrest::Postgrest;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct TokenInfo {
    pub address: String,
    pub decimals: i64,
    pub name: String,
    pub symbol: String,
    pub logo_uri: String,
}

pub async fn get_tokens(client: Postgrest) -> Result<Vec<TokenInfo>> {
    let resp = client
        .from("tokens")
        .select("*")
        .limit(100)
        .execute()
        .await?;

    Ok(resp.json().await?)
}
