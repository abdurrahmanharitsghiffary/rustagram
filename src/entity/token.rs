use std::fmt::Debug;

use chrono::NaiveDate;
use sqlx::{postgres::PgQueryResult, prelude::FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub token_type: TokenType,
    pub exp: i32,
    pub user_id: Uuid,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    ACCESS,
    REFRESH,
    VERIFY,
    RESET,
}

const QUERY_INSERT_TOKEN: &str =
    "INSERT INTO tokens (token, token_type, exp, user_id) VALUES (?, ?, ?, ?)";
const QUERY_DELETE_TOKEN: &str = "DELETE FROM tokens WHERE id = ?";
const QUERY_UPDATE_TOKEN: &str =
    "UPDATE tokens SET token = ?, token_type = ?, exp = ?, user_id = ? WHERE id = ?";
const QUERY_FIND_MANY_TOKEN: &str = "SELECT * FROM token ORDER BY created_at DESC";
const QUERY_FIND_ONE_TOKEN: &str = "SELECT * FROM tokens WHERE id = ? ORDER BY created_at DESC";

pub async fn save(token: Token, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(QUERY_INSERT_TOKEN)
        .bind(token.token)
        .bind(token.token_type)
        .bind(token.exp)
        .bind(token.user_id)
        .execute(pool)
        .await
}

pub async fn destroy(token: Token, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(QUERY_DELETE_TOKEN)
        .bind(token.id)
        .execute(pool)
        .await
}

pub async fn update(
    token_id: i32,
    new: Token,
    pool: &Pool<Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(QUERY_UPDATE_TOKEN)
        .bind(new.token)
        .bind(new.token_type)
        .bind(new.exp)
        .bind(new.user_id)
        .bind(token_id)
        .execute(pool)
        .await
}

pub async fn find_many(pool: &Pool<Postgres>) -> Result<Vec<Token>, sqlx::Error> {
    sqlx::query_as::<_, Token>(QUERY_FIND_MANY_TOKEN)
        .fetch_all(pool)
        .await
}

pub async fn find_one(token_id: Uuid, pool: &Pool<Postgres>) -> Result<Option<Token>, sqlx::Error> {
    sqlx::query_as::<_, Token>(QUERY_FIND_ONE_TOKEN)
        .bind(token_id)
        .fetch_optional(pool)
        .await
}
