use crate::api::controllers::base::ControllerState;
use crate::core_utils::errors::OurErrors;
use axum::extract::State;
use axum::{routing::post, Json, Router};
use log::info;
use serde_json::Value;

pub struct UserController {
    pub router: Router,
}

impl UserController {
    pub fn new() -> Self {
        UserController {
            router: Router::new().route("/update", post(Self::update)).with_state(ControllerState {
                ns: String::from("user-controller"),
            }),
        }
    }

    pub async fn update(State(state): State<ControllerState>) -> Result<Json<Value>, OurErrors> {
        info!(target: &state.ns, "Triggered update user");
        Err(OurErrors::UnAuthorization)
    }
}
