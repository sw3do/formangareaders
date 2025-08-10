use crate::config::Config;
use crate::database::Database;
use crate::error::{AppError, Result};
use crate::models::AuthResponse;
use crate::services::UserService;
use crate::utils::JwtService;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
    given_name: Option<String>,
    _family_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DiscordUserInfo {
    id: String,
    username: String,
    _discriminator: String,
    email: Option<String>,
    avatar: Option<String>,
    global_name: Option<String>,
}

#[derive(Clone)]
pub struct OAuthService {
    user_service: UserService,
    jwt_service: JwtService,
    _config: Config,
    google_client: BasicClient,
    discord_client: BasicClient,
}

impl OAuthService {
    pub fn new(db: Database, config: Config) -> Result<Self> {
        let user_service = UserService::new(db);
        let jwt_service = JwtService::new(&config.jwt_secret);

        let google_client = BasicClient::new(
            ClientId::new(config.google_client_id.clone()),
            Some(ClientSecret::new(config.google_client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).map_err(
                |e| AppError::Internal(anyhow::anyhow!("Invalid Google auth URL: {}", e)),
            )?,
            Some(
                TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string()).map_err(
                    |e| AppError::Internal(anyhow::anyhow!("Invalid Google token URL: {}", e)),
                )?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(format!(
                "{}/api/v1/auth/google/callback",
                config.backend_url
            ))
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Invalid Google redirect URL: {}", e))
            })?,
        );

        let discord_client = BasicClient::new(
            ClientId::new(config.discord_client_id.clone()),
            Some(ClientSecret::new(config.discord_client_secret.clone())),
            AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Invalid Discord auth URL: {}", e))
            })?,
            Some(
                TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).map_err(|e| {
                    AppError::Internal(anyhow::anyhow!("Invalid Discord token URL: {}", e))
                })?,
            ),
        )
        .set_redirect_uri(
            RedirectUrl::new(format!(
                "{}/api/v1/auth/discord/callback",
                config.backend_url
            ))
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Invalid Discord redirect URL: {}", e))
            })?,
        );

        Ok(Self {
            user_service,
            jwt_service,
            _config: config,
            google_client,
            discord_client,
        })
    }

    pub fn get_google_auth_url(&self) -> (String, String) {
        let (auth_url, csrf_token) = self
            .google_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        (auth_url.to_string(), csrf_token.secret().clone())
    }

    pub fn get_discord_auth_url(&self) -> (String, String) {
        let (auth_url, csrf_token) = self
            .discord_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("identify".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();

        (auth_url.to_string(), csrf_token.secret().clone())
    }

    pub async fn handle_google_callback(&self, code: &str) -> Result<AuthResponse> {
        let token_result = self
            .google_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| AppError::OAuth(format!("Failed to exchange Google code: {e}")))?;

        let access_token = token_result.access_token().secret();

        let client = reqwest::Client::new();
        let user_info: GoogleUserInfo = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?
            .json()
            .await?;

        let username = user_info
            .given_name
            .or(user_info.name.clone())
            .unwrap_or_else(|| format!("user_{}", &user_info.id[..8]));

        let avatar_url = user_info.picture;
        let display_name = user_info.name;

        let user = self
            .user_service
            .find_or_create_oauth_user(
                &user_info.email,
                &username,
                display_name,
                avatar_url,
                "google",
                &user_info.id,
            )
            .await?;

        let token = self.jwt_service.generate_token(user.id, &user.email)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    pub async fn handle_discord_callback(&self, code: &str) -> Result<AuthResponse> {
        let token_result = self
            .discord_client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| AppError::OAuth(format!("Failed to exchange Discord code: {e}")))?;

        let access_token = token_result.access_token().secret();

        let client = reqwest::Client::new();
        let user_info: DiscordUserInfo = client
            .get("https://discord.com/api/users/@me")
            .bearer_auth(access_token)
            .send()
            .await?
            .json()
            .await?;

        let email = user_info.email.ok_or_else(|| {
            AppError::OAuth("Discord account must have a verified email".to_string())
        })?;

        let username = user_info
            .global_name
            .clone()
            .or(Some(user_info.username.clone()))
            .unwrap_or_else(|| format!("user_{}", &user_info.id[..8]));

        let avatar_url = user_info.avatar.map(|avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                user_info.id, avatar
            )
        });

        let display_name = user_info.global_name.or(Some(user_info.username));

        let user = self
            .user_service
            .find_or_create_oauth_user(
                &email,
                &username,
                display_name,
                avatar_url,
                "discord",
                &user_info.id,
            )
            .await?;

        let token = self.jwt_service.generate_token(user.id, &user.email)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }
}
