use super::dto::{LoginDTO, RegisterDTO, ResetPasswordDTO};

pub async fn send_forgot_password_email(email: String) {}
pub async fn login(dto: LoginDTO) {}
pub async fn register_account(dto: RegisterDTO) {}
pub async fn reset_password(dto: ResetPasswordDTO) {}
pub async fn revoke_token(token: String) {}
