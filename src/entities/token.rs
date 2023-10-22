use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_derive_plus::SurrealDerive;

#[derive(Debug, Clone, Deserialize, Serialize, SurrealDerive)]
pub struct Token {
    pub content: String,
    pub created_at: DateTime<Utc>
}
