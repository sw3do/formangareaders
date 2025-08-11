use crate::config::Config;
use crate::database::Database;
use crate::handlers::auth::*;
use crate::middleware::auth::auth_middleware;
use crate::services::auth::AuthService;
use crate::services::oauth::OAuthService;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub oauth_service: OAuthService,
    pub config: Config,
}

pub fn create_auth_routes(db: Database) -> Router {
    let config = Config::from_env().expect("Failed to load configuration");

    let auth_service =
        AuthService::new(db.clone(), config.clone()).expect("Failed to create auth service");

    let oauth_service = OAuthService::new(db, config.clone()).expect("Failed to create oauth service");

    let app_state = AppState {
        auth_service: auth_service.clone(),
        oauth_service,
        config: config.clone(),
    };

    let protected_routes = Router::new()
        .route("/me", get(me))
        .route("/logout", post(logout))
        .route_layer(middleware::from_fn_with_state(
            auth_service,
            auth_middleware,
        ));

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify-email", post(verify_email))
        .route("/resend-verification", post(resend_verification))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/google", get(google_auth))
        .route("/google/callback", get(google_callback))
        .route("/discord", get(discord_auth))
        .route("/discord/callback", get(discord_callback))
        .merge(protected_routes)
        .with_state(app_state)
}
