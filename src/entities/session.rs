use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_derive_plus::SurrealDerive;
use crate::entities::token::Token;
use crate::entities::user::User;

/**
* The session is represent for the login session of user,
* and a session can be hashed into an access token
* every access token will be valid in a very short term (< 15min)
* after expired, the refresh token can generate a new access token
* but every refresh token can only be use once, after generate a new access token
* the current refresh token will become invalid,
* a new refresh token will be response along with the new access token.
*/
#[derive(Debug, Clone, Deserialize, Serialize, SurrealDerive)]
pub struct Session {
    pub refresh_token: Token,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub last_refreshed_at: DateTime<Utc>
}

impl Session {
    pub fn new(token: Token, user: User) -> Self {
        Self {
            refresh_token: token,
            user,
            created_at: Utc::now(),
            last_refreshed_at: token.created_at.clone(),
        }
    }
}