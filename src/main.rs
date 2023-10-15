extern crate proc_macro;
extern crate async_trait;
extern crate surreal_derive;

mod api;
mod core_utils;
mod entities;
mod grpc;
mod services;
mod db;

use api::controllers::authentication::AuthenticationController;
use axum::{Router, Server};
use log::info;
use crate::core_utils::configs::CONFIGS;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}};
use std::fmt::Display;
use std::ops::Deref;
use once_cell::sync::Lazy;

use pretty_env_logger::formatted_timed_builder;
use surreal_devl::config::SurrealDeriveConfig;
use surreal_devl::serialize::SurrealSerialize;
use crate::api::controllers::user::UserController;

type Db = Surreal<Client>;

static DB: Lazy<Db> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
    // #region setup environments
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

    // #region setup db connection
    DB.connect::<Ws>(CONFIGS.surreal_db.socket_address.clone()).await.expect("Failed while connecting to surreal db");
    DB.use_ns(CONFIGS.surreal_db.namespace.clone()).use_db(CONFIGS.surreal_db.db_name.clone()).await.unwrap();
    DB.signin(Root {
        username: CONFIGS.surreal_db.db_username.clone().as_str(),
        password: CONFIGS.surreal_db.db_password.clone().as_str() 
    }).await.unwrap();

    let db_version = DB.version().await.expect("Failed to get the surreal db version");

    info!(target: &namespace, "Connected to SurrealDb version:{} {}", db_version, CONFIGS.surreal_db.socket_address);
    // #endregion

    // #region start server
    info!(target: &namespace, "Starting Restful API server is started at port: {}", CONFIGS.restful_service.port);
    let main_route = Router::new()
        .merge(UserController::new(DB.clone()).router)
        .merge(AuthenticationController::new(DB.clone()).router);

    Server::bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), CONFIGS.restful_service.port))
        .serve(main_route.into_make_service())
        .await
        .unwrap();
    // #endregion
}
