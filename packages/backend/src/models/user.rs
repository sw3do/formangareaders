use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[derive(Default)]
pub enum UserRole {
    #[default]
    User,
    Moderator,
    Admin,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn is_moderator(&self) -> bool {
        matches!(self, UserRole::Moderator)
    }

    pub fn is_user(&self) -> bool {
        matches!(self, UserRole::User)
    }

    pub fn can_moderate(&self) -> bool {
        matches!(self, UserRole::Moderator | UserRole::Admin)
    }

    pub fn can_admin(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        matches!(
            (self, required_role),
            (UserRole::Admin, _)
                | (UserRole::Moderator, UserRole::User | UserRole::Moderator)
                | (UserRole::User, UserRole::User)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub verification_expires_at: Option<DateTime<Utc>>,
    pub reset_token: Option<String>,
    pub reset_expires_at: Option<DateTime<Utc>>,
    pub provider: String,
    pub provider_id: Option<String>,
    pub locale: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    pub fn is_moderator(&self) -> bool {
        self.role.is_moderator()
    }

    pub fn is_user(&self) -> bool {
        self.role.is_user()
    }

    pub fn can_moderate(&self) -> bool {
        self.role.can_moderate()
    }

    pub fn can_admin(&self) -> bool {
        self.role.can_admin()
    }

    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        self.role.has_permission(required_role)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub is_verified: bool,
    pub provider: String,
    pub locale: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            role: user.role,
            is_verified: user.is_verified,
            provider: user.provider,
            locale: user.locale,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub display_name: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: Option<String>,
}
