use surrealdb::sql::Id;
use crate::core_utils::errors::OurErrors;
use crate::db::base::{DbResource, IntoDbResource};
use crate::entities::issuer::Issuer;

impl IntoDbResource<Issuer> for Issuer {
    const TABLE_NAME: &'static str = "issuer";

    fn into_db_resource(self) -> Result<DbResource<Issuer>, OurErrors> {
        Ok(DbResource((Self::TABLE_NAME.to_owned(), Id::String(self.name.clone())), self))
    }
}