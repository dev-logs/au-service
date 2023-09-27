use serde::{Deserialize, Serialize};
use surrealdb::{
    opt::{IntoResource, RecordId, Resource},
    sql::Id,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl IntoResource<Option<User>> for User {
    fn into_resource(self) -> surrealdb::Result<Resource> {
        Ok(Resource::RecordId(RecordId {
            id: Id::String(self.name.clone()),
            tb: "user".to_owned(),
        }))
    }
}
