use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "[0-9A-Za-z]{8}-[0-9A-Za-z]{4}-4[0-9A-Za-z]{3}-[89ABab][0-9A-Za-z]{3}-[0-9A-Za-z]{12}",
    )
    .expect("Failed to create the REGEX")
});

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_min_length = password.len() >= 8;

    if has_digit && has_lowercase && has_uppercase && has_min_length {
        Ok(())
    } else {
        Err(ValidationError::new("Password must contain at least 8 characters, 1 Uppercase letter, 1 Lowercase letter, and 1 Number"))
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct ForgotPasswordRequestDTO {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct LoginDTO {
    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "validate_password",))]
    pub password: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ResetPasswordDTO {
    #[validate(regex(path= *UUID_REGEX, message="Invalid token"))]
    pub token: String,

    #[validate(custom(function = "validate_password",))]
    pub new_password: String,

    #[validate(must_match(other = "new_password"))]
    pub confirm_new_password: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterDTO {
    #[validate(email, length(max = 255))]
    pub email: String,

    #[validate(custom(function = "validate_password",))]
    pub password: String,

    #[validate(must_match(other = "password"))]
    pub confirm_password: String,

    #[validate(length(min = 2, max = 125))]
    pub first_name: String,

    #[validate(length(min = 2, max = 125))]
    pub last_name: Option<String>,

    #[validate(length(min = 2, max = 100))]
    pub username: String,
}
