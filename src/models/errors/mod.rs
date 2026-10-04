
#[derive(Debug, Error)]
pub enum Oauth2ClientServiceError {
    #[error("client name cannot be empty")]
    InvalidName,
    #[error("missing redirect uri")]
    MissingRedirectUri,
    #[error("invalid redirect uri")]
    InvalidRedirectUri,
    #[error("client already exists")]
    ClientAlreadyExists,
    #[error("client not found")]
    ClientNotFound,
    #[error("database connection error")]
    DatabaseError(#[from] sqlx::Error),
}

