
use axum::Error;
use uuid::Uuid;
use crate::models::requests::register_client::{RegisterClientRequest, RegisterClientResponse};
use crate::models::errors::ClientServiceError;
use crate::repository::DatabaseConnection;

pub struct ClientService {
    pub db: DatabaseConnection
}


impl ClientService {

    pub async fn register(
        &self, 
        request: RegisterClientRequest
    ) -> Result<RegisterClientResponse, ClientServiceError>  {

        let new_id = Uuid::new_v4();

        let db = self.db.new("sqlite::memory:");




        // generate hash
    }
}