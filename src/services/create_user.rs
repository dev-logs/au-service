use crate::Db;
use crate::core_utils::errors::OurErrors;
use crate::entities::user::User;
use crate::services::base::{OurService, OurResult};
use async_trait::async_trait;
use log::info;
use serde::{Deserialize, Serialize};
use surreal_derive::surreal_quote;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct CreateUserService {
   pub db: Db
}

#[derive(Debug, Clone)]
pub struct Params {
    pub user: User
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[async_trait]
impl OurService<Params, User> for CreateUserService {
    async fn execute(self, params: Params) -> OurResult<User> {
        info!("created user");
        if let Some(created_user) = self.db.query(surreal_quote!("CREATE #record(&params.user)")).await?.take(0)? {
           return Ok(created_user);
        }

       Err(OurErrors::UnAuthorization)
    }
}

