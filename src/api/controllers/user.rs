use crate::entities::user::User;
use crate::services::base::{OurService, OurResult};
use crate::services::create_user::CreateUserService;
use axum::extract::State;
use axum::{routing::post, Json, Router};
use lazy_static::__Deref;
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
            router: Router::new().route("/create", post(Self::create))
                .with_state(UserControllerState {
                    ns: String::from("user-controller"),
                    create_user_service: CreateUserService { db: db.clone() }
                })
        }
    }

    pub async fn create(State(state): State<UserControllerState>, create_user: Json<User>) -> OurResult<User> {
        Ok(state.create_user_service.execute(crate::services::create_user::Params { user: create_user.deref().clone() }).await?)
    }
}

