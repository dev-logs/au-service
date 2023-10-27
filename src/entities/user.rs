use serde::{Deserialize, Serialize};
use surreal_derive_plus::SurrealDerive;

#[derive(Debug, Serialize, Deserialize, SurrealDerive, Clone)]
pub struct User {
    pub name: String,
    pub full_name: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, full_name: String, password: String) -> Self {
        Self {
            name, full_name, password
        }
    }
}
