use serde::{Deserialize, Serialize};
use surreal_derive_plus::SurrealDerive;

#[derive(Debug, Serialize, Deserialize, SurrealDerive, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: String,
    pub full_name: String,
    pub password: String,
}
