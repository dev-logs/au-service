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
                Token {token: headers.get("Authorization").unwrap().to_str().unwrap().to_owned()}
            );
        }

        Err(OurErrors::UnAuthorization)
    }
}

