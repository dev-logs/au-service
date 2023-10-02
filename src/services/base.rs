use async_trait::async_trait;

use crate::core_utils::errors::OurErrors;

pub type NoParam = ();

pub type VoidResponse = ();

pub type OurResult<T> = Result<T, OurErrors>;

pub trait OurResponse {}

#[async_trait]
pub trait OurService<P, T>
where
    P: Clone,
{
    async fn execute(self, params: P) -> OurResult<T>;
}
