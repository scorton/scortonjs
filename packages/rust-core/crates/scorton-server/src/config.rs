use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub cors_origins: Vec<String>,
    pub rate_limit: u32,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("SCORTON_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SCORTON_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap_or(8000),
            jwt_secret: env::var("SCORTON_JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-key".to_string()),
            cors_origins: env::var("SCORTON_CORS_ORIGINS")
                .unwrap_or_else(|_| "*".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            rate_limit: env::var("SCORTON_RATE_LIMIT")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8000,
            jwt_secret: "default-secret-key".to_string(),
            cors_origins: vec!["*".to_string()],
            rate_limit: 1000,
        }
    }
}
