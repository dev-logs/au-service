use std::time::Duration;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use log::info;

use crate::{core_utils::errors::OurErrors, entities::token::Token};

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Token {
    type Rejection = OurErrors;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, OurErrors> {
        info!("->> {:<12} - Context", "EXTRACTOR");
        let headers = &parts.headers;
        if headers.contains_key("Authorization") {
            return Result::Ok(
                Token {content: headers.get("Authorization").unwrap().to_str().unwrap().to_owned(), created_at: Default::default(), duration: Duration::from_millis(200000) }
            );
        }

        Err(OurErrors::UnAuthorization)
    }
}

