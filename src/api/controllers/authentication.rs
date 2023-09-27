use axum::{Router, routing::post, extract::State, Json};
use lazy_static::__Deref;

use crate::Db;
use crate::services::base::{OurResult, OurService};
use crate::services::register;
use crate::entities::user::User;

pub struct AuthenticationController {
    pub router: Router,
}

#[derive(Clone)]
pub struct AuthenticationControllerState {
    pub ns: String,
    pub register_service: register::RegisterUserService
}

impl AuthenticationController {
    pub fn new(db: Db) -> Self {
        Self {
            router: Router::new()
                .route("/authentication/register", post(Self::register))
                .with_state(AuthenticationControllerState {
                    ns: "authentication".to_string(),
                    register_service: register::RegisterUserService { create_user_service: crate::services::create_user::CreateUserService { db: db.clone() } }
                })
        }
    }

    pub async fn register(State(state): State<AuthenticationControllerState>, user: Json<User>) -> OurResult<User> {
        state.register_service.execute(register::Params {user: user.deref().clone()}).await
    }
}
