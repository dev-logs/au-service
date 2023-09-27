use crate::Db;
use crate::core_utils::errors::OurErrors;
use crate::entities::user::User;
use crate::services::base::{OurService, OurResult};
use async_trait::async_trait;
use surrealdb::opt::IntoResource;

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
    async fn execute(&self, params: Params) -> OurResult<User> {
        if let Some(created_user) = self.db.create(params.user.clone()).content(params.user).await? {
           return Ok(created_user);
        }

        Result::Err(OurErrors::UnAuthorization)
    }
}

