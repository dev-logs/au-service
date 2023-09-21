use crate::core_utils::errors::OurErrors;
use crate::services::base::{NoParam, OurService, VoidResponse};
use async_trait::async_trait;

pub struct CreateUserService;

#[async_trait]
impl OurService<NoParam, VoidResponse> for CreateUserService {
    async fn execute(&self, _: NoParam) -> Result<VoidResponse, OurErrors> {
        Ok(())
    }
}
