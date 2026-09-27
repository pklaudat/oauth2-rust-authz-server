use axum::routing::{get, post, put, delete};

use crate::models::requests::register_client::{RegisterClientRequest, UpdateClientRequest};


#[utoipa::path(
    post,
    path = "/admin/application",
    tag = "Admin Console",
    responses(
        (status = 201, description = "Register new Oauth2 application")
    )
)]
pub async fn register_new_application(request: RegisterClientRequest) {
}


#[utoipa::path(
    put,
    path = "/admin/application/{id}",
    tag = "Admin Console",
    responses(
        (status = 204, description = "Update Oauth2 application")
    )
)]
pub async fn update_application(request: UpdateClientRequest) {
    
}

#[utoipa::path(
    delete,
    path = "/admin/application/{id}",
    tag = "Admin Console",
    responses(
        (status = 204, description = "Delete Oauth2 application")
    )
)]
pub async fn delete_application(id: String) {
    
}