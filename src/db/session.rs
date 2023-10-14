use std::collections::BTreeMap;
use surrealdb::opt::RecordId;
use surrealdb::sql::{Id, Object, Thing, Value};
use crate::entities::session::Session;

impl Into<Thing> for Session {
    fn into (self) -> RecordId{
        let mut objectId : BTreeMap<String, Value>= BTreeMap::new();
        objectId.insert("user_name".to_owned(), Value::from(self.user.name.clone()));
        objectId.insert("generated_at".to_owned(), Value::from(self.created_at.clone()));

        RecordId::from(("session", Id::Object(Object(objectId))))
    }
}
