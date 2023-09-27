use crate::entities::user::User;
use crate::services::base::{OurService, OurResult};
use async_trait::async_trait;
use super::create_user;

#[derive(Clone)]
pub struct RegisterUserService {
   pub create_user_service: create_user::CreateUserService
}

#[derive(Debug, Clone)]
pub struct Params {
    pub user: User
}

#[async_trait]
impl OurService<Params, User> for RegisterUserService {
    async fn execute(&self, params: Params) -> OurResult<User> {
        self.create_user_service.execute(create_user::Params {user: params.user}).await

    }
}

