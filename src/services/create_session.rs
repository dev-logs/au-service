use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::{Db, entities::{session::Session, user::User}, core_utils::errors::OurErrors, db::base::IntoDbResource};

use super::base::{OurService, OurResult};

pub struct CreateSessionService {
    db: Db,
}

pub struct Params {
    user_name: String,
    password: String
}

#[async_trait]
impl OurService<Params, Session> for CreateSessionService {
    async fn execute(self, params: Params) -> OurResult<Session> {
        let user: Option<User> = self.db.select(("user", params.user_name)).await?;

        if None = user {
            return Err(OurErrors::UnAuthorization);
        }

        let now = Utc::now();
        let new_session = Session {
            created_at: now.clone(),
            last_refreshed_at: now.clone(),
            expired_at: now.clone(),
            user: user.take()
        };

        let (session_db_key, session_db_value) = new_session.into_db_resource();
        let created_session: Option<Session> = self.db.create(session_db_key).content(session_db_value).await?;

        if None = created_session {
            return Err(OurErrors::InternalServerError("Not able to create new session, error code: 522".to_owned()));
        }

        Ok(created_session.take())
    }
}
