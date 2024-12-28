use std::fmt::Debug;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgQueryResult, FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::common::query::template::{InsertTemplate, UpdateTemplate};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub token_type: TokenType,
    pub exp: i32,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    ACCESS,
    REFRESH,
    VERIFY,
    RESET,
}

pub async fn save(token: Token, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(&InsertTemplate::new("tokens", &["token", "token_type", "exp", "user_id"]).query)
        .bind(token.token)
        .bind(token.token_type)
        .bind(token.exp)
        .bind(token.user_id)
        .execute(&*pool)
        .await
}

pub async fn destroy(token: Token, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM tokens WHERE id = ?")
        .bind(token.id)
        .execute(&*pool)
        .await
}

pub async fn update(
    token_id: i32,
    pool: &Pool<Postgres>,
    new: Token,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        &UpdateTemplate::new("id", "tokens", &["token", "token_type", "exp", "user_id"]).query,
    )
    .bind(new.token)
    .bind(new.token_type)
    .bind(new.exp)
    .bind(new.user_id)
    .execute(&*pool)
    .await
}

pub async fn find_many(pool: &Pool<Postgres>) -> Result<Vec<Token>, sqlx::Error> {
    sqlx::query_as::<_, Token>("SELECT * FROM tokens ORDER BY created_at DESC")
        .fetch_all(&*pool)
        .await
}

pub async fn find_one(pool: &Pool<Postgres>, token_id: Uuid) -> Result<Option<Token>, sqlx::Error> {
    sqlx::query_as::<_, Token>("SELECT * FROM tokens WHERE id = ? ORDER BY created_at DESC")
        .bind(token_id)
        .fetch_optional(&*pool)
        .await
}
