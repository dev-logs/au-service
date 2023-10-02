use crate::Db;
use crate::core_utils::errors::OurErrors;
use crate::entities::user::User;
use crate::services::base::{OurService, OurResult};
use async_trait::async_trait;
use crate::db::base::{DbResource, IntoDbResource};

#[derive(Clone)]
pub struct CreateUserService {
   pub db: Db
}

#[derive(Debug, Clone)]
pub struct Params {
    pub user: User
}

#[async_trait]
impl OurService<Params, User> for CreateUserService {
    async fn execute(self, params: Params) -> OurResult<User> {
        let vs: Vec<User> = vec! {User { name: "".to_string(), password: "".to_string() }};
        let DbResource(db, value) = params.user.into_db_resource()?;

        if let Some(created_user) = self.db.create(db).content(value).await? {
           return Ok(created_user);
        }

        Result::Err(OurErrors::UnAuthorization)
    }
}

