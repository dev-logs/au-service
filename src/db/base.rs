use surrealdb::{
    opt::{IntoResource, RecordId, Resource},
    sql::Id,
};

use crate::core_utils::errors::OurErrors;

pub struct DbResource<R>(pub (String, Id), pub R);

impl<R, T> IntoResource<R> for DbResource<R> {
    fn into_resource(self) -> surrealdb::Result<Resource> {
        Ok(Resource::RecordId(RecordId {
            tb: self.0 .0,
            id: self.0.1,
        }))
    }
}

pub trait IntoDbResource<R> {
    const TABLE_NAME: &'static str;
    fn into_db_resource(self) -> Result<DbResource<R>, OurErrors>;
}
