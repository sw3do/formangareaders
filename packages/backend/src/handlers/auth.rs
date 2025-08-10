use crate::error::{AppError, Result};
use crate::models::{
    ForgotPasswordRequest, LoginRequest, OAuthCallbackQuery, RegisterRequest,
    ResetPasswordRequest, User, UserResponse, VerifyEmailRequest,
};
use crate::routes::auth::AppState;
use axum::{
    extract::{Extension, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    Json,
};
use serde_json::json;

fn get_locale_from_headers(headers: &HeaderMap) -> String {
    headers
        .get("accept-language")
        .and_then(|h| h.to_str().ok())
        .and_then(|lang| lang.split(',').next())
        .and_then(|lang| lang.split('-').next())
        .map(|lang| match lang {
            "tr" => "tr".to_string(),
            _ => "en".to_string(),
        })
        .unwrap_or_else(|| "en".to_string())
}

pub async fn register(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    let locale = get_locale_from_headers(&headers);
    let user = app_state.auth_service.register(request, &locale).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "Registration successful. Please check your email to verify your account.",
            "user": user
        })),
    ))
}

pub async fn login(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    let locale = get_locale_from_headers(&headers);
    let response = app_state.auth_service.login(request, &locale).await?;

    Ok(Json(response))
}

pub async fn verify_email(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<VerifyEmailRequest>,
) -> Result<impl IntoResponse> {
    let locale = get_locale_from_headers(&headers);
    app_state
        .auth_service
        .verify_email(&request.token, &locale)
        .await?;

    Ok(Json(json!({
        "message": "Email verified successfully"
    })))
}

pub async fn resend_verification(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let locale = get_locale_from_headers(&headers);
    let email = request["email"]
        .as_str()
        .ok_or_else(|| AppError::Validation("Email is required".to_string()))?;

    app_state
        .auth_service
        .resend_verification(email, &locale)
        .await?;

    Ok(Json(json!({
        "message": "Verification email resent"
    })))
}

pub async fn forgot_password(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ForgotPasswordRequest>,
) -> Result<impl IntoResponse> {
    let locale = get_locale_from_headers(&headers);
    app_state
        .auth_service
        .forgot_password(&request.email, &locale)
        .await?;

    Ok(Json(json!({
        "message": "Password reset email sent if account exists"
    })))
}

pub async fn reset_password(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse> {
    let locale = get_locale_from_headers(&headers);
    app_state
        .auth_service
        .reset_password(&request.token, &request.new_password, &locale)
        .await?;

    Ok(Json(json!({
        "message": "Password reset successful"
    })))
}

pub async fn me(Extension(user): Extension<User>) -> Result<impl IntoResponse> {
    let user_response: UserResponse = user.into();
    Ok(Json(user_response))
}

pub async fn google_auth(State(app_state): State<AppState>) -> Result<impl IntoResponse> {
    let (auth_url, _csrf_token) = app_state.oauth_service.get_google_auth_url();
    Ok(Redirect::temporary(&auth_url))
}

pub async fn google_callback(
    State(app_state): State<AppState>,
    Query(params): Query<OAuthCallbackQuery>,
) -> Result<impl IntoResponse> {
    let auth_response = app_state
        .oauth_service
        .handle_google_callback(&params.code)
        .await?;

    let redirect_url = format!(
        "{}?token={}&user={}",
        "http://localhost:3000/auth/callback",
        auth_response.token,
        serde_json::to_string(&auth_response.user).unwrap_or_default()
    );

    Ok(Redirect::temporary(&redirect_url))
}

pub async fn discord_auth(State(app_state): State<AppState>) -> Result<impl IntoResponse> {
    let (auth_url, _csrf_token) = app_state.oauth_service.get_discord_auth_url();
    Ok(Redirect::temporary(&auth_url))
}

pub async fn discord_callback(
    State(app_state): State<AppState>,
    Query(params): Query<OAuthCallbackQuery>,
) -> Result<impl IntoResponse> {
    let auth_response = app_state
        .oauth_service
        .handle_discord_callback(&params.code)
        .await?;

    let redirect_url = format!(
        "{}?token={}&user={}",
        "http://localhost:3000/auth/callback",
        auth_response.token,
        serde_json::to_string(&auth_response.user).unwrap_or_default()
    );

    Ok(Redirect::temporary(&redirect_url))
}

pub async fn logout() -> Result<impl IntoResponse> {
    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}
