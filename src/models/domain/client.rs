use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};



#[derive(Debug, Clone, Serialize)]
pub struct Client {
    pub id: String,
    pub client_id: String,
    pub client_secret: String,
    pub client_secret_hash: Option<String>,
    pub grant_types: Vec<GrantType>,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    RefreshToken,
}
