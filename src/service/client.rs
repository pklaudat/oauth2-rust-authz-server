
use crate::models::requests::register_client::{RegisterClientRequest, RegisterClientResponse};
use crate::models::errors::ClientServiceError;
use crate::repository::Oauth2;
use crate::repository::client::{Oauth2ClientRepository};

pub struct Oauth2ClientService {
    pub client_repository: Oauth2ClientRepository,
}


impl Oauth2ClientService {

    pub async fn new(client_repository: Oauth2ClientRepository) -> Result<Self, ClientServiceError> {
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