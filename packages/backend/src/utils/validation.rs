use crate::error::{AppError, Result};
use validator::Validate;

pub fn validate_request<T: Validate>(request: &T) -> Result<()> {
    request.validate().map_err(|errors| {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!(
                        "{}: {}",
                        field,
                        error.message.as_ref().unwrap_or(&"Invalid value".into())
                    )
                })
            })
            .collect();

        AppError::Validation(error_messages.join(", "))
    })
}

pub fn is_valid_locale(locale: &str) -> bool {
    matches!(
        locale,
        "en" | "tr" | "es" | "fr" | "de" | "ja" | "ko" | "zh"
    )
}
