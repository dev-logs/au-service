use async_trait::async_trait;
use chrono::Utc;
use surreal_derive_plus::surreal_quote;
use crate::{Db, entities::{session::Session, user::User}, core_utils::errors::OurErrors};
use crate::entities::token::Token;
use super::base::{OurService, OurResult};

pub struct CreateSessionService {
    db: Db,
}

#[derive(Clone)]
pub struct Params {
    user_name: String,
    password: String
}

#[async_trait]
impl OurService<Params, Session> for CreateSessionService {
    async fn execute(self, params: Params) -> OurResult<Session> {
        let user: Option<User> = self.db.select(("user", params.user_name)).await?;

        if user.is_none() {
            return Err(OurErrors::UnAuthorization);
        }

        let now = Utc::now();
        let new_session = Session {
            current_refresh_token: Token {
                value: "".to_owned(),
                created_at: Default::default(),
            },
            created_at: now.clone(),
            last_refreshed_at: now.clone(),
            expired_at: now.clone(),
            user: user.unwrap()
        };

        let created_session: Option<Session> = self.db.query(surreal_quote!(r"
            CREATE #record(&new_session)
        ")).await?.take(0)?;

        if created_session.is_none() {
            return Err(OurErrors::InternalServerError { message: "Not able to create new session, error code: 522".to_owned() });
        }

        Ok(created_session.unwrap())
    }
}
