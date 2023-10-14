use serde::{Deserialize, Serialize};
use surreal_derive::SurrealDerive;

#[derive(Debug, Serialize, Deserialize, SurrealDerive, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
}
