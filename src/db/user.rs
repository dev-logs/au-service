use surrealdb::opt::RecordId;
use surrealdb::sql::Thing;
use crate::entities::user::User;

impl Into<Thing> for User {
    fn into(self) -> RecordId {
        return RecordId::from(("user", self.name.as_str()));
    }
}
