use axum::{http::StatusCode, response::IntoResponse};

use crate::entities::user::User;

impl IntoResponse for User {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, serde_json::to_string(&self).unwrap()).into_response()
    }
}
