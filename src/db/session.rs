use std::collections::BTreeMap;
use surrealdb::sql::{Id, Object, Value};
use crate::core_utils::errors::OurErrors;
use crate::db::base::{DbResource, IntoDbResource};
use crate::entities::session::Session;

impl IntoDbResource<Session> for Session {
    const TABLE_NAME: &'static str = "session";

    fn into_db_resource(self) -> Result<DbResource<Session>, OurErrors> {
        let mut objectId : BTreeMap<String, Value>= BTreeMap::new();
        objectId.insert("user_name".to_owned(), Value::from(self.user.name.clone()));
        objectId.insert("generated_at".to_owned(), Value::from(self.created_at.clone()));

        Ok(DbResource((Self::TABLE_NAME.to_owned(), Id::Object(Object(objectId))), self.clone()))
    }
}
