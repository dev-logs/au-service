extern crate proc_macro;
extern crate async_trait;
extern crate devl_macro;

mod api;
mod core_utils;
mod entities;
mod grpc;
mod services;
mod db;

use api::controllers::authentication::AuthenticationController;
use axum::{Router, Server};
use devl_macro::surreal_derive;
use log::info;
use crate::core_utils::configs::CONFIGS;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Root, sql::{Value, statements, Table, Data, Idiom}};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, collections::BTreeMap};
use std::fmt::Display;
use once_cell::sync::Lazy;

use pretty_env_logger::formatted_timed_builder;
use surrealdb::sql::{Operator, Statements};
use crate::api::controllers::user::UserController;

type Db = Surreal<Client>;

static DB: Lazy<Db> = Lazy::new(Surreal::init);

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
    DB.use_ns(CONFIGS.surreal_db.namespace.clone()).use_db(CONFIGS.surreal_db.db_name.clone()).await.unwrap();
    DB.signin(Root {
        username: CONFIGS.surreal_db.db_username.clone().as_str(),
        password: CONFIGS.surreal_db.db_password.clone().as_str() 
    }).await.unwrap();

    let db_version = DB.version().await.expect("Failed to get the surreal db version");

    info!(target: &namespace, "Connected to SurrealDb version:{} {}", db_version, CONFIGS.surreal_db.socket_address);
    // #endregion

    // #region restful service
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

#[derive(devl_macro::surreal_derive)]
pub struct PP {
    pub name: String,
    pub age: i32,

}

#[test]
fn test_derive() {
    println!("Tiendang-debug");
    let x = PP {name: String::from(""), age: 32};

    let create_command = statements::CreateStatement {
        what: surrealdb::sql::Values(vec![Value::Table(Table("".to_owned()))]),
        data: Some(Data::SetExpression(x.into_create_expressions())),
        ..Default::default()
    };

    // let update_command = statements::UpdateStatement {
    //     only: false,
    //     what: surrealdb::sql::Values(vec![Value::Table(Table("pp".to_owned()))]),
    //     data: Some(Data::UpdateExpression(x.into_update_expressions())),
    //     cond: None,
    //     output: None,
    //     timeout: None,
    //     parallel: false,
    // };

    println!("tiendang sql: {}", create_command);
}

