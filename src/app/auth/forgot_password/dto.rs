use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct ForgotPasswordRequestDTO {
    #[validate(email)]
    email: String,
}
