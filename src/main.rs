mod api;
mod core_utils;
mod entities;
mod grpc;
mod services;

use axum::{Router, Server};
use log::info;
use crate::core_utils::configs::CONFIGS;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use once_cell::sync::Lazy;

use pretty_env_logger::formatted_timed_builder;
use crate::api::controllers::user::UserController;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
    // #region environments
    let namespace = String::from("au-service");
    let mut log_builder = formatted_timed_builder();
    if let Ok(filter_env) = std::env::var("RUST_LOG") {
        log_builder.parse_filters(&filter_env);
    }
    else {
        log_builder.filter(None, log::LevelFilter::Info);
    }

    log_builder.init();
    info!(target: &namespace, "Configs {:?}", *CONFIGS);
    // #endregion

    // #region db
    DB.connect::<Ws>(CONFIGS.surreal_db.socket_address.clone()).await.expect("Failed while connecting to surreal db");
    let db_version = DB.version().await.expect("Failed to get the surreal db version");
    info!(target: &namespace, "Connected to SurrealDb version:{} at {}", db_version, CONFIGS.surreal_db.socket_address);
    // #endregion

    // #region restful service
    info!(target: &namespace, "Starting restful api server at port: {}", CONFIGS.restful_service.port);
    let main_route = Router::new().merge(UserController::new().router);
    Server::bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), CONFIGS.restful_service.port))
        .serve(main_route.into_make_service())
        .await
        .unwrap();
    // #endregion
}

