pub mod auth;

use crate::database::Database;
use axum::Router;

pub fn create_routes(db: Database) -> Router {
    Router::new().nest("/auth", auth::create_auth_routes(db))
}
