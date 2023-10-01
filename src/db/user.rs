use surrealdb::sql::Id;
use crate::core_utils::errors::OurErrors;
use crate::db::base::{DbResource, IntoDbResource};
use crate::entities::user::User;

impl IntoDbResource<User> for User {
    const TABLE_NAME: &'static str = "user";

    fn into_db_resource(self) -> Result<DbResource<User>, OurErrors> {
        Ok(DbResource((Self::TABLE_NAME.to_owned(), Id::String(self.name.clone())), self))
    }
}
