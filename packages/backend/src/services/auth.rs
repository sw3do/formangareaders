use crate::config::Config;
use crate::database::Database;
use crate::error::{AppError, Result};
use crate::i18n::I18n;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse};
use crate::services::UserService;
use crate::utils::{validate_request, verify_password, EmailService, JwtService};

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    jwt_service: JwtService,
    email_service: EmailService,
    config: Config,
    i18n: I18n,
}

impl AuthService {
    pub fn new(db: Database, config: Config) -> Result<Self> {
        let user_service = UserService::new(db);
        let jwt_service = JwtService::new(&config.jwt_secret);
        let email_service = EmailService::new(&config.smtp)?;
        let i18n = I18n::new();

        Ok(Self {
            user_service,
            jwt_service,
            email_service,
            config,
            i18n,
        })
    }

    pub async fn register(&self, request: RegisterRequest, _locale: &str) -> Result<UserResponse> {
        validate_request(&request)?;

        let user = self.user_service.create_user(request).await?;

        if let Some(verification_token) = &user.verification_token {
            self.email_service
                .send_verification_email(
                    &user.email,
                    &user.username,
                    verification_token,
                    &self.config.frontend_url,
                )
                .await
                .map_err(|e| {
                    tracing::error!("Failed to send verification email: {:?}", e);
                    AppError::Internal(anyhow::anyhow!("Failed to send verification email"))
                })?;
        }

        Ok(user.into())
    }

    pub async fn login(&self, request: LoginRequest, locale: &str) -> Result<AuthResponse> {
        validate_request(&request)?;

        let user = self
            .user_service
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| {
                AppError::Authentication(self.i18n.get_message(locale, "invalid-credentials", None))
            })?;

        if user.provider != "local" {
            return Err(AppError::Authentication(
                "Please use OAuth login for this account".to_string(),
            ));
        }

        let password_hash = user.password_hash.as_ref().ok_or_else(|| {
            AppError::Authentication(self.i18n.get_message(locale, "invalid-credentials", None))
        })?;

        if !verify_password(&request.password, password_hash)? {
            return Err(AppError::Authentication(self.i18n.get_message(
                locale,
                "invalid-credentials",
                None,
            )));
        }

        if !user.is_verified {
            return Err(AppError::Authentication(self.i18n.get_message(
                locale,
                "account-not-verified",
                None,
            )));
        }

        let token = self.jwt_service.generate_token(user.id, &user.email)?;

        Ok(AuthResponse {
            user: user.into(),
            token,
        })
    }

    pub async fn verify_email(&self, token: &str, locale: &str) -> Result<()> {
        let user = self
            .user_service
            .find_by_verification_token(token)
            .await?
            .ok_or_else(|| {
                AppError::Authentication(self.i18n.get_message(locale, "invalid-token", None))
            })?;

        self.user_service.verify_email(user.id).await?;

        Ok(())
    }

    pub async fn resend_verification(&self, email: &str, locale: &str) -> Result<()> {
        let user = self
            .user_service
            .find_by_email(email)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(self.i18n.get_message(locale, "user-not-found", None))
            })?;

        if user.is_verified {
            return Err(AppError::Validation(
                "Email is already verified".to_string(),
            ));
        }

        let verification_token = self.user_service.update_verification_token(user.id).await?;

        self.email_service
            .send_verification_email(
                &user.email,
                &user.username,
                &verification_token,
                &self.config.frontend_url,
            )
            .await?;

        Ok(())
    }

    pub async fn forgot_password(&self, email: &str, locale: &str) -> Result<()> {
        if let Some(user) = self.user_service.find_by_email(email).await? {
            if user.provider != "local" {
                return Err(AppError::Validation(self.i18n.get_message(
                    locale,
                    "oauth-password-reset-not-allowed",
                    None,
                )));
            }
        }

        if let Some(reset_token) = self.user_service.create_reset_token(email).await? {
            let user = self.user_service.find_by_email(email).await?.unwrap();

            self.email_service
                .send_password_reset_email(
                    &user.email,
                    &user.username,
                    &reset_token,
                    &self.config.frontend_url,
                )
                .await?;
        }

        Ok(())
    }

    pub async fn reset_password(
        &self,
        token: &str,
        new_password: &str,
        locale: &str,
    ) -> Result<()> {
        if new_password.len() < 8 {
            return Err(AppError::Validation(self.i18n.get_message(
                locale,
                "password-too-short",
                None,
            )));
        }

        let success = self
            .user_service
            .reset_password(token, new_password)
            .await?;

        if !success {
            return Err(AppError::Authentication(self.i18n.get_message(
                locale,
                "invalid-token",
                None,
            )));
        }

        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<User> {
        let claims = self.jwt_service.verify_token(token)?;

        let user = self
            .user_service
            .find_by_id(claims.user_id)
            .await?
            .ok_or_else(|| AppError::Authentication("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn update_locale(&self, user_id: uuid::Uuid, locale: &str) -> Result<crate::models::UserResponse> {
        let user = self.user_service.update_locale(user_id, locale).await?;
        Ok(user.into())
    }
}