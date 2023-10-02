use surrealdb::opt::Resource::RecordId;
use surrealdb::sql::{Id, Thing};
use crate::core_utils::errors::OurErrors;
use crate::entities::token::Token;

use crate::db::base::DbResource;
use crate::db::base::IntoDbResource;

impl IntoDbResource<Token> for Token {
    const TABLE_NAME: &'static str = "token";

    fn into_db_resource(self) -> Result<DbResource<Token>, OurErrors> {
        Ok(DbResource((Self::TABLE_NAME.to_owned(), Id::String(self.value.clone())), self))
    }
}
