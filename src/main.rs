use std::net::SocketAddr;
use axum::{Router, routing::get};
use std::env;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod service;
mod repository;

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

#[tokio::main]
async fn main() {
    dotenv().ok();

    match env::var("DATABASE_URL") {
        Ok(val) => println!("Database is set: {}", val),
        Err(e) => println!("Error reading database configuration {}", e),
    }

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
