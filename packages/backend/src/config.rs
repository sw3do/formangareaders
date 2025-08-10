use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub app_name: String,
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub smtp: SmtpConfig,
    pub frontend_url: String,
    pub backend_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            port: env::var("PORT")?.parse().unwrap_or(8000),
            database_url: env::var("DATABASE_URL")?,
            app_name: env::var("APP_NAME")?,
            jwt_secret: env::var("JWT_SECRET_KEY")?,
            google_client_id: env::var("GOOGLE_CLIENT_ID")?,
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")?,
            discord_client_id: env::var("DISCORD_CLIENT_ID")?,
            discord_client_secret: env::var("DISCORD_CLIENT_SECRET")?,
            smtp: SmtpConfig {
                host: env::var("SMTP_HOST")?,
                port: env::var("SMTP_PORT")?.parse().unwrap_or(587),
                username: env::var("SMTP_USERNAME")?,
                password: env::var("SMTP_PASSWORD")?,
                from_email: env::var("SMTP_FROM_EMAIL")?,
                from_name: env::var("SMTP_FROM_NAME")?,
            },
            frontend_url: env::var("FRONTEND_URL")?,
            backend_url: env::var("BACKEND_URL")?,
        })
    }
}
