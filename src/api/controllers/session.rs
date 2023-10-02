use axum::{Router, routing::post, extract::State, Json};
use axum::extract::RawBody;
use lazy_static::__Deref;
use surrealdb::dbs::Session;

use crate::Db;
use crate::entities::token::Token;
use crate::services::base::{OurResult, OurService};
use crate::services::register;
use crate::entities::user::User;

pub struct SessionController {
    pub router: Router,
}

#[derive(Clone)]
pub struct SessionControllerState {
    pub ns: String,
    pub register_service: register::RegisterUserService
}

impl SessionController {
    pub fn new(db: Db) -> Self {
        Self {
            router: Router::new()
                .route("/session", post(Self::create))
                .with_state(SessionControllerState {
                    ns: "session-controller".to_string(),
                    register_service: register::RegisterUserService { create_user_service: crate::services::create_user::CreateUserService { db: db.clone() } }
                })
        }
    }

    pub async fn create(State(state): State<SessionControllerState>, user: Json<User>) -> OurResult<User> {
        state.register_service.execute(register::Params {user: user.deref().to_owned()}).await
    }
}
