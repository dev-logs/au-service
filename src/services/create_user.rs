use crate::Db;
use crate::entities::user::User;
use crate::core_utils::errors::OurErrors;
use crate::services::base::{OurService, VoidResponse};
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
impl OurService<Params, VoidResponse> for CreateUserService {
    async fn execute(&self, params: Params) -> Result<VoidResponse, OurErrors> {
        let _: Vec<User> = self.db.create("user").content(params.user).await?;
        Ok(())
    }
}

