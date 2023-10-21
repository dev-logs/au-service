use surrealdb::opt::RecordId;
use crate::entities::user::User;

impl Into<RecordId> for User {
    fn into(self) -> RecordId {
        return RecordId::from(("user", self.name.as_str()));
    }
}
