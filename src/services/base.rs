use async_trait::async_trait;

use crate::core_utils::errors::OurErrors;

pub type NoParam = ();

pub type VoidResponse = ();

pub trait OurResponse {}

#[async_trait]
pub trait OurService<P, T>
where
    P: Clone + Copy,
{
    async fn execute(&self, params: P) -> Result<T, OurErrors>;
}
