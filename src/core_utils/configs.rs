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
    /// https://surrealdb.com/docs/surrealql/statements/define/namespace
    pub namespace: String,
    pub db_name: String,
    pub db_password: String,
    pub db_username: String,
}

impl Default for SurrealDb {
    fn default() -> Self {
        SurrealDb {
            socket_address: env::var("DEVLOGS_SURREAL_DB_SOCKET_ADDRESS").unwrap_or("127.0.0.1:8000".to_owned()),
            db_name: env::var("DEVLOGS_SURREAL_DB_NAME").unwrap_or("AU".to_owned()),
            namespace: env::var("DEVLOGS_SURREAL_DB_NAMESPACE").unwrap_or("AU".to_owned()),
            db_username: env::var("DEVLOGS_SURREAL_DB_USERNAME").unwrap_or("root".to_owned()),
            db_password: env::var("DEVLOGS_SURREAL_DB_PASSWORD").unwrap_or("root".to_owned()),
        }
    }
}

#[derive(Debug)]
pub struct TokenConfig {
    pub jwt_refresh_token_private_key: String,
    pub jwt_access_token_private_key: String,
    pub access_token_duration: u64,
    pub refresh_token_duration: u64,
    pub session_duration: u64
}

impl Default for TokenConfig {
    fn default() -> Self {
        let refresh_token_duration = env::var("DEVLOGS_AU_REFRESH_TOKEN_LIFETIME_IN_MS")
            .map(|env_var| env_var.parse().expect("The DEVLOGS_AU_REFRESH_TOKEN_LIFETIME_IN_MS must be number"))
            .unwrap_or(86400000); // 1 day
        Self {
            jwt_refresh_token_private_key: env::var("DEVLOGS_AU_JWT_REFRESH_TOKEN_PRIVATE_KEY")
                .unwrap_or("this_is_unsafe_keythis_is_unsafe_keythis_is_unsafe_key".to_owned()),
            jwt_access_token_private_key: env::var("DEVLOGS_AU_JWT_ACCESS_TOKEN_PRIVATE_KEY")
                .unwrap_or("this_is_unsafe_keythis_is_unsafe_keythis_is_unsafe_key".to_owned()),
            access_token_duration: env::var("DEVLOGS_AU_ACCESS_TOKEN_LIFETIME_IN_MS")
                .map(|env_var| env_var.parse().expect("The DEVLOGS_AU_ACCESS_TOKEN_LIFETIME_IN_MS must be number"))
                .unwrap_or(900000), // 15 minutes
            refresh_token_duration,
            session_duration: env::var("DEVLOGS_AU_SESSION_LIFETIME_IN_MS")
                .map(|env_var| env_var.parse().expect("The DEVLOGS_AU_SESSION_LIFETIME_IN_MS must be number"))
                .unwrap_or(refresh_token_duration) // equal the refresh token duration
        }
    }
}

#[derive(Debug, Default)]
pub struct Config {
    pub surreal_db: SurrealDb,
    pub grpc_service: GrpcService,
    pub restful_service: RestfulService,
    pub token: TokenConfig,
}

lazy_static! {
    pub static ref CONFIGS: Config = Default::default();
}
