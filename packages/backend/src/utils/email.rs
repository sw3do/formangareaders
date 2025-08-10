use crate::config::SmtpConfig;
use crate::error::{AppError, Result};
use lettre::message::{header::ContentType, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Clone)]
pub struct EmailService {
    mailer: SmtpTransport,
    from_email: Mailbox,
}

impl EmailService {
    pub fn new(config: &SmtpConfig) -> Result<Self> {
        let creds = Credentials::new(config.username.clone(), config.password.clone());

        let mailer = SmtpTransport::relay(&config.host)?
            .port(config.port)
            .credentials(creds)
            .build();

        let from_email = format!("{} <{}>", config.from_name, config.from_email)
            .parse()
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid from email: {}", e)))?;

        Ok(Self { mailer, from_email })
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        username: &str,
        verification_token: &str,
        frontend_url: &str,
    ) -> Result<()> {
        let verification_url =
            format!("{frontend_url}/verify-email?token={verification_token}");

        let html_body = format!(
            r#"
            <html>
                <body style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                    <div style="background-color: #f8f9fa; padding: 20px; border-radius: 10px;">
                        <h2 style="color: #333; text-align: center;">Email Verification</h2>
                        <p>Hello <strong>{username}</strong>,</p>
                        <p>Thank you for registering with ForMangaReaders! Please click the button below to verify your email address:</p>
                        <div style="text-align: center; margin: 30px 0;">
                            <a href="{verification_url}" style="background-color: #007bff; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block;">Verify Email</a>
                        </div>
                        <p>If the button doesn't work, you can copy and paste this link into your browser:</p>
                        <p style="word-break: break-all; color: #666;">{verification_url}</p>
                        <p style="color: #666; font-size: 12px; margin-top: 30px;">This link will expire in 24 hours. If you didn't create an account, please ignore this email.</p>
                    </div>
                </body>
            </html>
            "#
        );

        let email = Message::builder()
            .from(self.from_email.clone())
            .to(to_email
                .parse()
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid to email: {}", e)))?)
            .subject("Verify your email address")
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        username: &str,
        reset_token: &str,
        frontend_url: &str,
    ) -> Result<()> {
        let reset_url = format!("{frontend_url}/reset-password?token={reset_token}");

        let html_body = format!(
            r#"
            <html>
                <body style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto;">
                    <div style="background-color: #f8f9fa; padding: 20px; border-radius: 10px;">
                        <h2 style="color: #333; text-align: center;">Password Reset</h2>
                        <p>Hello <strong>{username}</strong>,</p>
                        <p>You requested to reset your password. Click the button below to set a new password:</p>
                        <div style="text-align: center; margin: 30px 0;">
                            <a href="{reset_url}" style="background-color: #dc3545; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block;">Reset Password</a>
                        </div>
                        <p>If the button doesn't work, you can copy and paste this link into your browser:</p>
                        <p style="word-break: break-all; color: #666;">{reset_url}</p>
                        <p style="color: #666; font-size: 12px; margin-top: 30px;">This link will expire in 1 hour. If you didn't request a password reset, please ignore this email.</p>
                    </div>
                </body>
            </html>
            "#
        );

        let email = Message::builder()
            .from(self.from_email.clone())
            .to(to_email
                .parse()
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Invalid to email: {}", e)))?)
            .subject("Reset your password")
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        self.mailer.send(&email)?;
        Ok(())
    }
}
