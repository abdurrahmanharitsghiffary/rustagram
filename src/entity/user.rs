use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::prelude::FromRow;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::common::query::template::{InsertTemplate, UpdateTemplate};

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

pub async fn save(user: User, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        &InsertTemplate::new(
            "users",
            &[
                "username",
                "email",
                "password_hash",
                "first_name",
                "last_name",
            ],
        )
        .query,
    )
    .bind(user.username)
    .bind(user.email)
    .bind(user.password_hash)
    .bind(user.first_name)
    .bind(user.last_name)
    .execute(&*pool)
    .await
}

pub async fn destroy(user: User, pool: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user.id)
        .execute(&*pool)
        .await
}

pub async fn update(
    user_id: Uuid,
    pool: &Pool<Postgres>,
    new: User,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        &UpdateTemplate::new(
            "id",
            "users",
            &[
                "username",
                "email",
                "password_hash",
                "first_name",
                "last_name",
            ],
        )
        .query,
    )
    .bind(new.username)
    .bind(new.email)
    .bind(new.password_hash)
    .bind(new.first_name)
    .bind(new.last_name)
    .bind(user_id)
    .execute(&*pool)
    .await
}

pub async fn find_many(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&*pool)
        .await
}

pub async fn find_one(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ? ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_optional(&*pool)
        .await
}
