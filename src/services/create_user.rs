use tower_http::services;

use crate::core_utils::errors::OurErrors;
use crate::services::base::{NoParam, OurResponse, OurService};
use async_trait::async_trait;

pub struct CreateUserService;

pub struct Params {
    user_name: String,
}

pub struct Response {}

#[async_trait]
impl OurService<NoParam, Response> for CreateUserService {
    async fn execute(&self, _: NoParam) -> Result<Response, OurErrors> {
        Ok(Response {})
    }
}
