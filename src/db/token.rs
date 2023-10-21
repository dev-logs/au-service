use surrealdb::opt::RecordId;
use crate::entities::token::Token;

impl Into<RecordId> for Token {
    fn into(self) -> RecordId {
        RecordId::from(("token", self.value.as_str()))
    }
}
