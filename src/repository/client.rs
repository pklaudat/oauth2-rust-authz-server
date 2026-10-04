
use azure_data_tables::clients::TableClient;
use azure_storage::Error;
use uuid::Uuid;
use crate::models::{domain::client::{Client, Oauth2Client}, requests::register_client::{RegisterClientRequest, RegisterClientResponse}};
pub struct ClientRepository {
    pub db_pool: TableClient<>,
}

impl ClientRepository {
    pub async fn new(db_pool: TableClient ) -> Result<Self, Error> {
        Ok(Self { db_pool })
    }

    pub async fn register_application(&self, request: RegisterClientRequest) -> Result<RegisterClientResponse, Error> {

        let id = Uuid::new_v4();

        let client_id: String = Uuid::new_v4().to_string();

        let client_secret: String = Uuid::new_v4().to_string();

        let entity = Oauth2Client{
            id: id.to_string(),
            client_id: client_id,
            client_secret: client_secret,
            client_secret_hash: None,
            name: request.name,
            redirect_uris: request.redirect_uris,
            grant_types: request.grant_types,
            scopes: request.scopes,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        self.db_pool.insert(&entity)?.await?;
        
        Ok(RegisterClientResponse {
            client_id: client_id,
            client_secret: Some(client_secret),
        })
    }

    pub async fn query_application_details(&self, client_id: &str) {

    }

    pub async fn query_applications(&self) {

    }

}