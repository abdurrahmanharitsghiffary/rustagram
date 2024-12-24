use super::dto::{LoginDTO, RegisterDTO, ResetPasswordDTO};

pub async fn send_forgot_password_email(email: String) {}
pub async fn send_sucess_login_email(email: String, username: String) {}
pub async fn send_sucess_register_email(email: String, username: String) {}
pub async fn send_sucess_reset_password_email(email: String, username: String) {}

pub async fn login(dto: LoginDTO) {}
pub async fn register_account(dto: RegisterDTO) {}
pub async fn reset_password(dto: ResetPasswordDTO) {}
pub async fn revoke_token(token: String) {}
pub async fn get_user_details(sub: String) {}
