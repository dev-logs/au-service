use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub value: String,
    pub created_at: DateTime<Utc>
}
