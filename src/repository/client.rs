
use uuid::Uuid;

use crate::{models::requests::register_client::{RegisterClientRequest, RegisterClientResponse}, repository::DatabaseConnection};

pub struct ClientRepository {
    pub db_connection: DatabaseConnection,
}

impl ClientRepository {
    pub async fn new(db_connection: DatabaseConnection) -> Result<Self, sqlx::Error> {
        Ok(Self { db_connection })
    }

    pub async fn register_application(&self, request: RegisterClientRequest) -> Result<RegisterClientResponse, sqlx::Error> {

        let id = Uuid::new_v4();

        let client_id: String = Uuid::new_v4().to_string();

        let client_secret: String = Uuid::new_v4().to_string();

        let redirect_uris = serde_json::to_string(&request.redirect_uris)
            .expect("failed to deserialize redirect uris");

        let scopes = serde_json::to_string(&request.scopes)
            .expect("failed to deserialize the scopes");

        let created_at = chrono::Utc::now();

        sqlx::query(
            r#"
            INSERT INTO clients (
                id,
                client_id,
                client_secret,
                client_secret_hash,
                name,
                redirect_uris,
                scopes,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(id.to_string())
        .bind(client_id)
        .bind(client_secret)
        .bind("")
        .bind(request.name)
        .bind(redirect_uris)
        .bind(scopes)
        .bind(created_at.to_rfc3339())
        .execute(&self.db_connection.pool)
        .await?;

        
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