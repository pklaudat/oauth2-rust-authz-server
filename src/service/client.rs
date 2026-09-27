
use std::error::Error;

use axum::Error;
use uuid::Uuid;
use crate::models::requests::register_client::{RegisterClientRequest, RegisterClientResponse};
use crate::models::errors::ClientServiceError;
use crate::repository::DatabaseConnection;
use crate::repository::client::{ClientRepository};

pub struct ClientService {
    pub client_repository: ClientRepository,
}


impl ClientService {

    pub async fn new(db_connection: DatabaseConnection) -> Result<Self, sqlx::Error> {
        let client_repository = ClientRepository::new(db_connection).await?;
        Ok(Self { client_repository })
    }

    pub async fn register(
        &self, 
        request: RegisterClientRequest
    ) -> Result<RegisterClientResponse, ClientServiceError>  {

        let response = self
            .client_repository
            .register_application(request)
            .await?;


        Ok(response)
    }
}