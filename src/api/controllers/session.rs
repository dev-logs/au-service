use axum::{Router, routing::post, extract::State, Json};
use lazy_static::__Deref;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::Db;
use crate::entities::session::Session;
use crate::entities::token::Token;
use crate::entities::user::User;
use crate::services::base::{OurResult, OurService};
use crate::services::create_session;

pub struct SessionController {
    pub router: Router,
}

#[derive(Clone)]
pub struct SessionControllerState {
    pub ns: String,
    pub create_session_service: create_session::CreateSessionService
}

impl SessionController {
    pub fn new(db: Surreal<Client>) -> Self {
        Self {
            router: Router::new()
                .route("/session", post(Self::create))
                .with_state(SessionControllerState {
                    ns: "session-controller".to_string(),
                    create_session_service: create_session::CreateSessionService { db: db.clone() }
                })
        }
    }

    pub async fn create(State(state): State<SessionControllerState>, params: Json<create_session::Params>) -> OurResult<Json<Session>> {
        let create_session = state.create_session_service.execute(params.deref().clone()).await?;
        Ok(Json(create_session))
    }
}
