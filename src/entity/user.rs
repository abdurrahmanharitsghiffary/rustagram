use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

const QUERY_INSERT_USER: &str = "INSERT INTO users (username, email, password_hash, first_name, last_name) VALUES (?, ?, ?, ?, ?)";
const QUERY_DELETE_USER: &str = "DELETE FROM users WHERE id = ?";
const QUERY_UPDATE_USER: &str = "UPDATE users SET username = ?, email = ?, password_hash = ?, first_name = ?, last_name = ? WHERE id = ?";
const QUERY_FIND_MANY_USERS: &str = "SELECT * FROM users ORDER BY created_at DESC";
const QUERY_FIND_ONE_USER: &str = "SELECT * FROM users WHERE id = ? ORDER BY created_at DESC";

pub async fn save(user: User, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(QUERY_INSERT_USER)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.first_name)
        .bind(user.last_name)
        .execute(pool)
        .await
}

pub async fn destroy(user: User, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(QUERY_DELETE_USER)
        .bind(user.id)
        .execute(pool)
        .await
}

pub async fn update(
    user_id: Uuid,
    new: User,
    pool: &Pool<Postgres>,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(QUERY_UPDATE_USER)
        .bind(new.username)
        .bind(new.email)
        .bind(new.password_hash)
        .bind(new.first_name)
        .bind(new.last_name)
        .bind(user_id)
        .execute(pool)
        .await
}

pub async fn find_many(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(QUERY_FIND_MANY_USERS)
        .fetch_all(pool)
        .await
}

pub async fn find_one(user_id: Uuid, pool: &Pool<Postgres>) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(QUERY_FIND_ONE_USER)
        .bind(user_id)
        .fetch_optional(pool)
        .await
}
