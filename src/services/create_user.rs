use crate::Db;
use crate::entities::user::User;
use crate::services::base::{OurService, OurResult};
use async_trait::async_trait;

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
        let created_users: Vec<User> = self.db.create("user").content(params.user).await?;
        Ok(created_users.first().unwrap().clone())
    }
}

