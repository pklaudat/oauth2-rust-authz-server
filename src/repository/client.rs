
use uuid::Uuid;
use crate::models::requests::register_client::{RegisterClientRequest, RegisterClientResponse};

pub struct ClientRepository {
    pub db_pool: sqlx::SqlitePool,
}

impl ClientRepository {
    pub async fn new(db_pool: sqlx::SqlitePool) -> Result<Self, sqlx::Error> {
        Ok(Self { db_pool })
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

        sqlx::query!(
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
            VALUES (?,?,?,?,?,?,?,?)
            "#,
                id.to_string(),
                client_id,
                client_secret,
                request.name,
                redirect_uris,
                scopes,
                created_at.to_rfc3339()
        )
            .execute(&self.db_connection)
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