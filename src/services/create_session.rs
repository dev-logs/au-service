use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surreal_derive_plus::surreal_quote;
use crate::{Db, entities::{session::Session, user::User}, core_utils::errors::OurErrors};
use crate::entities::token::Token;
use super::base::{OurService, OurResult};

#[derive(Clone)]
pub struct CreateSessionService {
    pub(crate) db: Db,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub user_name: String,
    pub password: String
}

#[async_trait]
impl OurService<Params, Session> for CreateSessionService {
    async fn execute(self, params: Params) -> OurResult<Session> {
        let user: Option<User> = self.db.query(surreal_quote!(r###"
            SELECT * from user where name = #val(&params.user_name) and password = #val(&params.password)
        "###)).await?.take(0).unwrap();

        if user.is_none() {
            return Err(OurErrors::UnAuthorization);
        }

        let now = Utc::now();
        let new_session = Session {
            refresh_token: Token {
                content: surrealdb::sql::Uuid::new().to_string(),
                created_at: Default::default(),
                duration: std::time::Duration::from_millis(2000)
            },
            created_at: now.clone(),
            last_refreshed_at: now.clone(),
            user: user.unwrap()
        };

        let created_session: Option<Session> = self.db
            .query(surreal_quote!(r"
                BEGIN TRANSACTION;
                CREATE #record(&new_session.refresh_token);
                CREATE #record(&new_session);
                COMMIT TRANSACTION;
                SELECT * FROM #id(&new_session) FETCH refresh_token, user;
            "))
            .await?
            .take(2)?;

        if created_session.is_none() {
            return Err(OurErrors::InternalServerError { message: "Not able to create new session, error code: 522".to_owned() });
        }

        Ok(created_session.unwrap())
    }
}
