use crate::error::AppError;
use crate::services::AuthService;
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(auth_service): State<AuthService>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Authentication("Missing or invalid authorization header".to_string())
        })?;

    let user = auth_service.verify_token(auth_header).await?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(auth_service): State<AuthService>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(auth_header) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
    {
        if let Ok(user) = auth_service.verify_token(auth_header).await {
            request.extensions_mut().insert(user);
        }
    }

    next.run(request).await
}
