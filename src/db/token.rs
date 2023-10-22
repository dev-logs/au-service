use surrealdb::opt::RecordId;
use crate::entities::token::Token;

impl Into<RecordId> for Token {
    fn into(self) -> RecordId {
        RecordId::from(("token", self.content.as_str()))
    }
}
