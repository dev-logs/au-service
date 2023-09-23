use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OurErrors {
    #[error("You are not authrorized to access/perform this resource/action")]
    UnAuthorization,
    #[error("Internal server error {}", .0)]
    DbError(#[from] surrealdb::Error),
}

impl IntoResponse for OurErrors {
    fn into_response(self) -> axum::response::Response {
        match self {
            OurErrors::UnAuthorization => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
            OurErrors::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
        }
    }
}
