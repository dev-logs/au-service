mod api;
mod core_utils;
mod entities;
mod grpc;
mod services;

use axum::{Router, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use pretty_env_logger::formatted_timed_builder;
use crate::api::controllers::user::UserController;

#[tokio::main]
async fn main() {
    let mut log_builder = formatted_timed_builder();
    if let Ok(filter_env) = std::env::var("RUST_LOG") {
        log_builder.parse_filters(&filter_env);
    }
    else {
        log_builder.filter(None, log::LevelFilter::Info);
    }

    log_builder.init();

    let main_route = Router::new().merge(UserController::new().router);
    Server::bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000))
        .serve(main_route.into_make_service())
        .await
        .unwrap();
}
