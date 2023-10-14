use surrealdb::opt::RecordId;
use surrealdb::sql::Thing;
use crate::entities::issuer::Issuer;

impl Into<Thing> for Issuer {
    fn into(self) -> RecordId {
        RecordId::from(("issuer", self.name.as_str()))
    }
}