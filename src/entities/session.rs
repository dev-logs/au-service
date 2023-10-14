use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_derive::SurrealDerive;
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
    pub current_refresh_token: Token,
    pub last_refreshed_at: DateTime<Utc>,
    pub user: User,
    pub created_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
}
