use log::info;
use surreal_derive_plus::surreal_quote;
use crate::Db;
use crate::services::base::{OurResult, OurService};
use crate::entities::issuer::Issuer;

pub struct CreateIssuerService {
    db: Db
}

#[derive(Clone)]
pub struct Params {
    issuer: Issuer,
    ns: String
}

#[async_trait::async_trait]
impl OurService<Params, Issuer> for CreateIssuerService {
    async fn execute(self, params: Params) -> OurResult<Issuer> {
        let created_issuer: Option<Issuer> = self.db.query(surreal_quote!(r"CREATE #record(&params.issuer)")).await?.take(0)?;

        info!(target: &params.ns.clone(), "Created a new issuer {:?}", created_issuer);

        Ok(created_issuer.unwrap())
    }
}
