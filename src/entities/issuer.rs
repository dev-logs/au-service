use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_derive::SurrealDerive;

#[derive(Deserialize, Serialize, Debug, Clone, SurrealDerive)]
pub struct Issuer {
    pub name: String,
    pub base_uri: Option<String>,
    pub access_token_private_key: String,
    pub refresh_token_private_key: String,
    pub expiration_date: DateTime<Utc>
}