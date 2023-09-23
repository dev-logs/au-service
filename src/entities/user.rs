use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<SurrealDbId>,
    pub name: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, Clone)]
pub struct SurrealDbId {
    pub id: RecordId,
}

impl Serialize for SurrealDbId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.id.id.to_raw())
    }
}

impl<'de> Deserialize<'de> for SurrealDbId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            id: RecordId::deserialize(deserializer)?,
        })
    }
}
