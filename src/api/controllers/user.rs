use crate::core_utils::errors::OurErrors;
use crate::entities::user::User;
use crate::services::base::OurService;
use crate::services::create_user::CreateUserService;
use axum::extract::State;
use axum::{routing::post, Json, Router};
use lazy_static::__Deref;
use serde_json::Value;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub struct UserController {
    pub router: Router,
}

#[derive(Clone)]
pub struct UserControllerState {
    pub ns: String,
    pub create_user_service: CreateUserService
}

impl UserController {
    pub fn new(db: Surreal<Client>) -> Self {
        UserController {
            router: Router::new().route("/update", post(Self::create))
                .with_state(UserControllerState {
                    ns: String::from(""),
                    create_user_service: CreateUserService { db: db.clone() }
                }),
        }
    }

    pub async fn create(State(state): State<UserControllerState>, create_user: Json<User>) -> Result<(), OurErrors> {
        Ok(state.create_user_service.execute(crate::services::create_user::Params { user: create_user.deref().clone() }).await?)
    }
}

