use lazy_static::lazy_static;
use std::env;

#[derive(Debug)]
pub struct RestfulService {
    pub port: u16,
}

impl Default for RestfulService {
    fn default() -> Self {
        RestfulService {
            port: env::var("DEVLOGS_AU_RESTFUL_PORT")
                .map(|env_var| env_var.parse().expect("The DEVLOGS_AU_RESTFUL_PORT must be number"))
                .unwrap_or(3000),
        }
    }
}

#[derive(Debug)]
pub struct GrpcService {
    pub port: u16,
}

impl Default for GrpcService {
    fn default() -> Self {
        GrpcService {
            port: env::var("DEVLOGS_AU_GRPC_PORT")
                .map(|env_var| env_var.parse().expect("The DEVLOGS_AU_GRPC_PORT must be number"))
                .unwrap_or(50008),
        }
    }
}

#[derive(Debug)]
pub struct SurrealDb {
    pub socket_address: String,
}

impl Default for SurrealDb {
    fn default() -> Self {
        SurrealDb {
            socket_address: env::var("DEVLOGS_SURREAL_DB_SOCKET_ADDRESS").unwrap_or("127.0.0.1:8000".to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub surreal_db: SurrealDb,
    pub grpc_service: GrpcService,
    pub restful_service: RestfulService,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            surreal_db: Default::default(),
            restful_service: Default::default(),
            grpc_service: Default::default(),
        }
    }
}

lazy_static! {
    pub static ref CONFIGS: Config = Default::default();
}
