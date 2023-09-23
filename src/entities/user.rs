use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
}
