use surrealdb::opt::RecordId;
use surrealdb::sql::Thing;
use crate::entities::token::Token;

impl Into<Thing> for Token {
    fn into(self) -> RecordId {
        RecordId::from(("token", self.value.as_str()))
    }
}
