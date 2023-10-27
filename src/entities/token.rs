use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use surreal_derive_plus::SurrealDerive;

#[derive(Debug, Clone, Deserialize, Serialize, SurrealDerive)]
pub struct Token {
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub duration: Duration
}

impl Token {
    pub fn new(content: &str, duration: Duration) -> Self {
        Self {
            content: content.to_owned(),
            duration,
            created_at: Utc::now()
        }
    }
}
