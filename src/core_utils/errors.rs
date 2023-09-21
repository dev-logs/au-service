use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum OurErrors {
    #[error("Failed to perform database command")]
    DbExecuteCommandError,
    #[error("You are not authrorized to access")]
    UnAuthorization,
}

impl IntoResponse for OurErrors {
    fn into_response(self) -> axum::response::Response {
        match self {
            OurErrors::DbExecuteCommandError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
            OurErrors::UnAuthorization => (StatusCode::UNAUTHORIZED, self.to_string()).into_response(),
        }
    }
}
