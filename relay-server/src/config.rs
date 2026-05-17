use std::env;

#[derive(Clone)]
pub struct Config {
    pub relay_addr: String,
    pub grpc_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            relay_addr: env::var("RELAY_HOST")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            grpc_port: env::var("RELAY_GRPC_PORT")
                .unwrap_or_else(|_| "9000".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_default(),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            log_level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
        })
    }
}
