use log::info;
use crate::Db;
use crate::db::base::{DbResource, IntoDbResource};
use crate::services::base::{OurResult, OurService};
use crate::entities::issuer::Issuer;

pub struct CreateIssuerService {
    db: Db
}

pub struct Params {
    issuer: Issuer,
    ns: String
}

impl OurService<Params, Issuer> for CreateIssuerService {
    async fn execute(&self, params: Params) -> OurResult<Issuer> {
        let DbResource(issuer_key, issuer_content) = params.issuer.into_db_resource()?;
        let created_issuer = self.db.create(issuer_key).content(issuer_content).await?;

        info!(target: params.ns.clone(), "Created a new issuer {:?}", created_issuer);
        Ok(created_issuer)
    }
}