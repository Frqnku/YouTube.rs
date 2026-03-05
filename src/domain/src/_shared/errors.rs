use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Error: {0}")]
    Unexpected(#[from] anyhow::Error),
}