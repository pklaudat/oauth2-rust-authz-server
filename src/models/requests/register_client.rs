use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::models::domain::client::GrantType;


#[derive(Debug, Deserialize)]
pub struct RegisterClientRequest {
    pub name: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<GrantType>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RegisterClientResponse {
    pub client_id: String,
    pub client_secret: Option<String>,
}