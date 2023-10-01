use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub created_at: DateTime<Utc>
}
