use std::net::SocketAddr;
use axum::{Router, routing::get};
use azure_data_tables::clients::TableServiceClient;
use azure_storage::StorageCredentials;
use std::env;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{repository::{DatabaseConnection, client::Oauth2ClientRepository}, service::client::Oauth2ClientService};

mod models;
mod service;
mod repository;
mod routes;

#[derive(OpenApi)]
#[openapi(paths(
    health,
))]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/Health",
    responses(
        (status = 200, description = "Health Check path")
    ),
    tag = "health_check"
)]
async fn health() -> &'static str {
    "health"
}

struct StorageConnectionProperties {
    name: String,
    key: String,
    table_name: String
}

struct AppState {
    oauth2_repository: Oauth2ClientRepository,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let storage = StorageConnectionProperties{
        name: env::var("STORAGE_ACCOUNT_NAME").expect("Set the storage account name!."),
        key: env::var("STORAGE_ACCOUNT_KEY").expect("Set the storage account key for authentication!."),
        table_name: env::var("TABLE_NAME").expect("Set the table name for the oauth2 apps.")
    };

    let storage_credentials = StorageCredentials::access_key(storage.name.clone(), storage.key);

    let table_service = TableServiceClient::new(storage.name, storage_credentials);

    let table_client = table_service.table_client(storage.table_name);

    table_client.create().await?;

    let repository = Oauth2ClientRepository::new(table_client).await?;

    let service: Oauth2ClientService::new(repository);

    let app = Router::new()
        .route("/health", get(health))
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
