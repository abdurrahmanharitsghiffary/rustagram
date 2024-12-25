use std::fmt::Debug;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: u32,
    pub token: String,
    pub token_type: TokenType,
    pub exp: u32,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum TokenType {
    ACCESS,
    REFRESH,
    VERIFY,
    RESET,
}

impl TokenType {
    pub fn value(&self) -> &'static str {
        match *self {
            Self::ACCESS => "acc_tkn",
            Self::REFRESH => "rsh_tkn",
            Self::VERIFY => "vry_tkn",
            TokenType::RESET => "rst_tkn",
        }
    }
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ACCESS => write!(f, "ACCESS_TOKEN"),
            Self::REFRESH => write!(f, "REFRESH_TOKEN"),
            Self::VERIFY => write!(f, "VERIFY_TOKEN"),
            Self::RESET => write!(f, "RESET_TOKEN"),
        }
    }
}

impl Serialize for TokenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.value())
    }
}

impl<'de> Deserialize<'de> for TokenType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = Deserialize::deserialize(deserializer)?;
        match value {
            "acc_tkn" => Ok(TokenType::ACCESS),
            "rsh_tkn" => Ok(TokenType::REFRESH),
            "vry_tkn" => Ok(TokenType::VERIFY),
            "rst_tkn" => Ok(TokenType::RESET),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid token type: {}",
                value
            ))),
        }
    }
}
