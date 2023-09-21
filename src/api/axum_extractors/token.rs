use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{core_utils::errors::OurErrors, entities::token::Token};

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Token {
    type Rejection = OurErrors;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, OurErrors> {
        println!("->> {:<12} - Context", "EXTRACTOR");
        let result = parts
            .extensions
            .get::<Result<Token, OurErrors>>()
            .ok_or(OurErrors::UnAuthorization)?
            .clone();

        result.clone()
    }
}
