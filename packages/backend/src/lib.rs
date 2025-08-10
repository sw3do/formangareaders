pub mod config;
pub mod database;
pub mod error;
pub mod handlers;
pub mod i18n;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

pub use config::Config;
pub use database::Database;
pub use error::{AppError, Result};

use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub async fn create_app(_config: Config, db: Database) -> Router {
    Router::new()
        .nest("/api/v1", routes::create_routes(db.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
