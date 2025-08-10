use crate::database::Database;
use crate::error::{AppError, Result};
use crate::models::{RegisterRequest, User};
use crate::utils::{generate_verification_token, hash_password};
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, request: RegisterRequest) -> Result<User> {
        let existing_email = sqlx::query("SELECT id FROM users WHERE email = $1")
            .bind(&request.email)
            .fetch_optional(self.db.pool())
            .await?;

        if existing_email.is_some() {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        let existing_username = sqlx::query("SELECT id FROM users WHERE username = $1")
            .bind(&request.username)
            .fetch_optional(self.db.pool())
            .await?;

        if existing_username.is_some() {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }

        let password_hash = hash_password(&request.password)?;
        let verification_token = generate_verification_token();
        let verification_expires_at = Utc::now() + Duration::hours(24);
        let locale = request.locale.unwrap_or_else(|| "en".to_string());

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email, username, password_hash, display_name, 
                verification_token, verification_expires_at, locale
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(&request.email)
        .bind(&request.username)
        .bind(&password_hash)
        .bind(&request.display_name)
        .bind(&verification_token)
        .bind(verification_expires_at)
        .bind(&locale)
        .fetch_one(self.db.pool())
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.db.pool())
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db.pool())
            .await?;

        Ok(user)
    }

    pub async fn find_by_verification_token(&self, token: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE verification_token = $1 AND verification_expires_at > NOW()",
        )
        .bind(token)
        .fetch_optional(self.db.pool())
        .await?;

        Ok(user)
    }

    pub async fn verify_email(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users 
            SET is_verified = true, verification_token = NULL, verification_expires_at = NULL
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    pub async fn update_verification_token(&self, user_id: Uuid) -> Result<String> {
        let verification_token = generate_verification_token();
        let verification_expires_at = Utc::now() + Duration::hours(24);

        sqlx::query(
            r#"
            UPDATE users 
            SET verification_token = $1, verification_expires_at = $2
            WHERE id = $3
            "#,
        )
        .bind(&verification_token)
        .bind(verification_expires_at)
        .bind(user_id)
        .execute(self.db.pool())
        .await?;

        Ok(verification_token)
    }

    pub async fn create_reset_token(&self, email: &str) -> Result<Option<String>> {
        let reset_token = generate_verification_token();
        let reset_expires_at = Utc::now() + Duration::hours(1);

        let result = sqlx::query(
            r#"
            UPDATE users 
            SET reset_token = $1, reset_expires_at = $2
            WHERE email = $3
            "#,
        )
        .bind(&reset_token)
        .bind(reset_expires_at)
        .bind(email)
        .execute(self.db.pool())
        .await?;

        if result.rows_affected() > 0 {
            Ok(Some(reset_token))
        } else {
            Ok(None)
        }
    }

    pub async fn reset_password(&self, token: &str, new_password: &str) -> Result<bool> {
        let password_hash = hash_password(new_password)?;

        let result = sqlx::query(
            r#"
            UPDATE users 
            SET password_hash = $1, reset_token = NULL, reset_expires_at = NULL
            WHERE reset_token = $2 AND reset_expires_at > NOW()
            "#,
        )
        .bind(&password_hash)
        .bind(token)
        .execute(self.db.pool())
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find_or_create_oauth_user(
        &self,
        email: &str,
        username: &str,
        display_name: Option<String>,
        avatar_url: Option<String>,
        provider: &str,
        provider_id: &str,
    ) -> Result<User> {
        if let Some(user) = self.find_by_email(email).await? {
            if user.provider == provider && user.provider_id.as_deref() == Some(provider_id) {
                return Ok(user);
            }

            if user.provider == "local" {
                let updated_user = sqlx::query_as::<_, User>(
                    r#"
                    UPDATE users 
                    SET provider = $1, provider_id = $2, is_verified = true,
                        display_name = COALESCE($3, display_name),
                        avatar_url = COALESCE($4, avatar_url)
                    WHERE id = $5
                    RETURNING *
                    "#,
                )
                .bind(provider)
                .bind(provider_id)
                .bind(&display_name)
                .bind(&avatar_url)
                .bind(user.id)
                .fetch_one(self.db.pool())
                .await?;

                return Ok(updated_user);
            }
        }

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                email, username, display_name, avatar_url, 
                is_verified, provider, provider_id
            )
            VALUES ($1, $2, $3, $4, true, $5, $6)
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(username)
        .bind(&display_name)
        .bind(&avatar_url)
        .bind(provider)
        .bind(provider_id)
        .fetch_one(self.db.pool())
        .await?;

        Ok(user)
    }
}
